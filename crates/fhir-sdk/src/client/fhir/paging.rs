//! FHIR paging functionality, e.g. for search results.

use std::{any::type_name, fmt::Debug, marker::PhantomData};

use futures::{Stream, StreamExt, TryStreamExt, stream};
use reqwest::{StatusCode, Url, header::HeaderValue};

use super::{Client, Error};
use crate::{
	extensions::{BundleEntryExt, BundleEntryRequestExt, BundleExt, SearchEntryModeExt},
	version::FhirVersion,
};

/// Type alias for the `BundleEntry` type for any version.
type BundleEntry<V> = <<V as FhirVersion>::Bundle as BundleExt>::Entry;

/// Wrapper around `Bundle`s that have multiple pages of results, e.g. search results, resource
/// history, etc.
pub struct Page<V: FhirVersion, R> {
	/// The FHIR client to make further requests for the next pages and resources.
	client: Client<V>,
	/// The inner Bundle result.
	bundle: V::Bundle,
	/// The correlation ID to send when fetching all further pages.
	correlation_id: HeaderValue,

	/// The resource type to return in matches.
	_resource_type: PhantomData<R>,
}

impl<V: FhirVersion, R> Page<V, R>
where
	(StatusCode, V::OperationOutcome): Into<Error>,
	R: TryFrom<V::Resource> + Send + Sync + 'static,
	for<'a> &'a R: TryFrom<&'a V::Resource>,
{
	/// Create a new `Page` result from a `Bundle` and client.
	pub(crate) const fn new(
		client: Client<V>,
		bundle: V::Bundle,
		correlation_id: HeaderValue,
	) -> Self {
		Self { client, bundle, correlation_id, _resource_type: PhantomData }
	}

	/// Get the next page URL, if there is one.
	pub fn next_page_url(&self) -> Option<&String> {
		self.bundle.next_page_url()
	}

	/// Fetch the next page and return it.
	pub async fn next_page(&self) -> Option<Result<Self, Error>> {
		let next_page_url = self.next_page_url()?;
		let url = match Url::parse(next_page_url) {
			Ok(url) => url,
			Err(_err) => return Some(Err(Error::UrlParse(next_page_url.clone()))),
		};

		tracing::debug!("Fetching next page from URL: {next_page_url}");
		let next_bundle = match self
			.client
			.read_generic::<V::Bundle>(url, Some(self.correlation_id.clone()))
			.await
		{
			Ok(Some(bundle)) => bundle,
			Ok(None) => return Some(Err(Error::ResourceNotFound(next_page_url.clone()))),
			Err(err) => return Some(Err(err)),
		};

		Some(Ok(Self::new(self.client.clone(), next_bundle, self.correlation_id.clone())))
	}

	/// Get the `total` field, indicating the total number of results.
	pub fn total(&self) -> Option<u32> {
		self.bundle.total()
	}

	/// Get access to the inner `Bundle`.
	pub const fn bundle(&self) -> &V::Bundle {
		&self.bundle
	}

	/// Consume the `Page` and return the inner `Bundle`.
	pub fn into_inner(self) -> V::Bundle {
		self.bundle
	}

	/// Consumes the raw inner entries, leaving the page empty. Returns the entries.
	pub fn take_entries(&mut self) -> Vec<Option<BundleEntry<V>>> {
		self.bundle.take_entries()
	}

	/// Get the entries of this page, ignoring entries whenever there is no `resource` in the entry.
	pub fn entries(&self) -> impl Iterator<Item = &V::Resource> + Send {
		self.bundle.entries().filter_map(|entry| entry.resource())
	}

	/// Get the matches of this page, ignoring entries whenever there is no `resource` in the entry
	/// or resources of the wrong type.
	pub fn matches(&self) -> impl Iterator<Item = &R> + Send {
		self.bundle
			.entries()
			.filter(|entry| entry.search_mode().is_some_and(SearchEntryModeExt::is_match))
			.filter_map(|entry| entry.resource())
			.filter_map(|resource| resource.try_into().ok())
	}

	/// Get the entries of this page, where the `fullUrl` is automatically resolved whenever there
	/// is no `resource` in the entry. Delete entries are ignored as well (e.g. in history
	/// requests), you can access the raw entries with [Self::bundle] or [Self::take_entries] if you
	/// need these.
	/// Consumes the entries, leaving the page empty.
	pub fn entries_owned(
		&mut self,
	) -> impl Stream<Item = Result<V::Resource, Error>> + Send + 'static {
		let client = self.client.clone();
		let correlation_id = self.correlation_id.clone();
		stream::iter(self.take_entries().into_iter().flatten()).filter_map(move |entry| {
			resolve_bundle_entry(entry, client.clone(), correlation_id.clone())
		})
	}

	/// Get the matches of this page, where the `fullUrl` is automatically resolved whenever there
	/// is no `resource` in the entry. Delete entries are ignored as well (e.g. in history
	/// requests), you can access the raw entries with [Self::bundle] or [Self::take_entries] if you
	/// need these. Ignores entries of the wrong resource type and entries without resource or full
	/// URL.
	/// Consumes the entries, leaving the page empty.
	pub fn matches_owned(&mut self) -> impl Stream<Item = Result<R, Error>> + Send + 'static {
		let client = self.client.clone();
		let correlation_id = self.correlation_id.clone();
		stream::iter(
			self.take_entries()
				.into_iter()
				.flatten()
				.filter(|entry| entry.search_mode().is_some_and(SearchEntryModeExt::is_match)),
		)
		.filter_map(move |entry| {
			resolve_bundle_entry(entry, client.clone(), correlation_id.clone())
		})
		.try_filter_map(|resource| std::future::ready(Ok(resource.try_into().ok())))
	}

	/// Start automatic paging through all entries across pages.
	///
	/// Hint: you can activate pre-fetching by [StreamExt::buffered].
	pub fn all_entries(
		mut self,
	) -> impl Stream<Item = Result<V::Resource, Error>> + Send + 'static {
		self.entries_owned()
			.chain(
				stream::once(async move { self.next_page().await })
					.filter_map(std::future::ready)
					.map_ok(Self::all_entries)
					.try_flatten(),
			)
			.boxed() // Somehow gives error when using if not boxed?
	}

	/// Start automatic paging through all matches across pages.
	///
	/// Hint: you can activate pre-fetching by [StreamExt::buffered].
	pub fn all_matches(mut self) -> impl Stream<Item = Result<R, Error>> + Send + 'static {
		self.matches_owned()
			.chain(
				stream::once(async move { self.next_page().await })
					.filter_map(std::future::ready)
					.map_ok(Self::all_matches)
					.try_flatten(),
			)
			.boxed() // Somehow gives error when using if not boxed?
	}
}

/// Convert the bundle entry into a resource, resolving the `fullUrl` if there is no resource
/// inside and it is not a `DELETE` request. Returns `None` if there is neither resource nor full
/// URL or it is a `DELETE` request.
async fn resolve_bundle_entry<V: FhirVersion>(
	entry: BundleEntry<V>,
	client: Client<V>,
	correlation_id: HeaderValue,
) -> Option<Result<V::Resource, Error>>
where
	(StatusCode, V::OperationOutcome): Into<Error>,
{
	if entry.resource().is_some() {
		return entry.into_resource().map(Ok);
	}

	if let Some(request) = entry.request() {
		if request.is_delete() {
			return None;
		}
	}

	let full_url = entry.full_url()?;
	let url = match Url::parse(full_url) {
		Ok(url) => url,
		Err(_err) => return Some(Err(Error::UrlParse(full_url.clone()))),
	};

	let result = client
		.read_generic::<V::Resource>(url, Some(correlation_id))
		.await
		.and_then(|opt| opt.ok_or_else(|| Error::ResourceNotFound(full_url.clone())));
	Some(result)
}

impl<V: FhirVersion, R> Clone for Page<V, R> {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			bundle: self.bundle.clone(),
			correlation_id: self.correlation_id.clone(),
			_resource_type: self._resource_type,
		}
	}
}

impl<V: FhirVersion, R> Debug for Page<V, R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Page")
			.field("client", &self.client)
			.field("bundle", &self.bundle)
			.field("_resource_type", &type_name::<R>())
			.finish()
	}
}

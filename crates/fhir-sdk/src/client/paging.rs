//! FHIR paging functionality, e.g. for search results.

use std::{collections::VecDeque, pin::Pin, task::Poll};

use futures::{future::BoxFuture, ready, FutureExt, Stream};
use model::{
	codes::LinkRelationTypes,
	resources::{Bundle, BundleEntry, Resource},
};
use reqwest::Url;
use serde::de::DeserializeOwned;

use super::{model, Client, Error};

/// Results of a query that can be paged or given via URL only. The resources
/// can be consumed via the `Stream`/`StreamExt` traits.
pub struct Paged {
	/// The FHIR client to make further requests for the next pages.
	client: Client,
	/// The URL of the next page. This is opaque and can be anything the server
	/// wants. The client ensures it accesses the same server only.
	next_url: Option<Url>,
	/// The current set of entries cached.
	entries: VecDeque<BundleEntry>,
	/// Current future to retrieve a resource.
	future_resource: Option<BoxFuture<'static, Result<Resource, Error>>>,
	/// Current future to retrieve the next page.
	future_next_page: Option<BoxFuture<'static, Result<Bundle, Error>>>,
}

impl Paged {
	/// Start up a new Paged stream.
	pub(crate) fn new(client: Client, url: Url) -> Self {
		let next_url = Some(url);

		Self {
			client,
			next_url,
			entries: VecDeque::new(),
			future_resource: None,
			future_next_page: None,
		}
	}
}

impl Stream for Paged {
	type Item = Result<Resource, Error>;

	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		// Check on single resource future first to output the next resource.
		if let Some(future_resource) = self.future_resource.as_mut() {
			let item = ready!(future_resource.as_mut().poll(cx));
			self.future_resource = None;
			return Poll::Ready(Some(item));
		}

		// Otherwise check on next page future to save the next batch of entries.
		if let Some(future_next_page) = self.future_next_page.as_mut() {
			if let Poll::Ready(next_page) = future_next_page.as_mut().poll(cx) {
				self.future_next_page = None;

				// Get the Bundle or error out.
				let bundle = match next_page {
					Ok(bundle) => bundle,
					Err(err) => return Poll::Ready(Some(Err(err))),
				};

				// Parse the next page's URL or error out.
				if let Some(next_url_string) = find_next_page_url(&bundle) {
					let Ok(next_url) = Url::parse(next_url_string) else {
						return Poll::Ready(Some(Err(Error::UrlParse(next_url_string.clone()))));
					};
					self.next_url = Some(next_url);
				}

				// Save the `BundleEntry`s.
				self.entries.extend(bundle.0.entry.into_iter().flatten());
			}
		}

		// If there are not enough items in the queue, query the next page.
		if self.entries.len() < 5 {
			if let Some(next_page_url) = self.next_url.take() {
				self.future_next_page =
					Some(fetch_resource(self.client.clone(), next_page_url).boxed());
				cx.waker().wake_by_ref();
			}
		}

		// Then get the next item from the queue that has a resource of the correct
		// type.
		while let Some(entry) = self.entries.pop_front() {
			if let Some(resource) = entry.resource {
				return Poll::Ready(Some(Ok(resource)));
			} else if let Some(url) = entry.full_url {
				if let Ok(url) = Url::parse(&url) {
					self.future_resource = Some(fetch_resource(self.client.clone(), url).boxed());
					cx.waker().wake_by_ref();
					return Poll::Pending;
				}
			}
		}

		// Else check if all resources were consumed or if we are waiting for new
		// resources to arrive.
		if self.future_next_page.is_some() {
			Poll::Pending
		} else if self.next_url.is_some() {
			cx.waker().wake_by_ref();
			Poll::Pending
		} else {
			Poll::Ready(None)
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if self.next_url.is_some() {
			(self.entries.len(), None)
		} else {
			(self.entries.len(), Some(self.entries.len()))
		}
	}
}

/// Find the URL of the next page of the results returned in the Bundle.
fn find_next_page_url(bundle: &Bundle) -> Option<&String> {
	bundle
		.link
		.iter()
		.flatten()
		.find(|link| link.relation == LinkRelationTypes::Next)
		.map(|link| &link.url)
}

/// Query a resource from a given URL.
async fn fetch_resource<R: DeserializeOwned>(client: Client, url: Url) -> Result<R, Error> {
	// Make sure we are not forwarded to any malicious server.
	if url.origin() != client.0.base_url.origin() {
		return Err(Error::DifferentOrigin(url.to_string()));
	}

	// Fetch a single resource from the given URL.
	let request = client.0.client.get(url);
	let response = client.request_settings().make_request(request).await?;
	if response.status().is_success() {
		let resource: R = response.json().await?;
		Ok(resource)
	} else {
		Err(Error::from_response(response).await)
	}
}

impl std::fmt::Debug for Paged {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Paged")
			.field("client", &self.client)
			.field("next_url", &self.next_url)
			.field("entries", &self.entries)
			.field("future_resource", &self.future_resource.as_ref().map(|_| ()))
			.field("future_next_page", &self.future_next_page.as_ref().map(|_| ()))
			.finish()
	}
}

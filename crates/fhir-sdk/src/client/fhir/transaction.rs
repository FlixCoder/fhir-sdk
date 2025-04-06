//! Implementation of building batch/transaction requests and processing the
//! response.

use reqwest::{
	StatusCode,
	header::{self, HeaderValue},
};
use uuid::Uuid;

use super::{Client, Error};
use crate::{
	extensions::{BundleEntryExt, BundleEntryRequestExt, BundleExt, GenericResource},
	version::FhirVersion,
};

/// Type alias for the `BundleEntry` type for any version.
type BundleEntry<V> = <<V as FhirVersion>::Bundle as BundleExt>::Entry;
/// Type alias for the `BundleEntryRequest` type for any version.
type BundleEntryRequest<V> =
	<<<V as FhirVersion>::Bundle as BundleExt>::Entry as BundleEntryExt>::Request;

/// A batch/transaction request builder.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the batch/transaction"]
pub struct BatchTransaction<V: FhirVersion> {
	/// The FHIR client.
	client: Client<V>,
	/// If this is a transaction. Otherwise it is a batch request.
	is_transaction: bool,
	/// Current entries in the batch or transaction.
	entries: Vec<Option<BundleEntry<V>>>,
}

impl<V: FhirVersion> BatchTransaction<V>
where
	(StatusCode, V::OperationOutcome): Into<Error>,
{
	/// Create new batch or transaction builder, given whether it is a
	/// transaction.
	pub(crate) const fn new(client: Client<V>, is_transaction: bool) -> Self {
		Self { client, is_transaction, entries: Vec::new() }
	}

	/// Add creation of a resource to the batch/transaction.
	/// For advanced features like conditional creation via `ifNoneExists`,
	/// please use `with_raw` for now.
	///
	/// Returns the temporary UUID to be used a reference that will be resolved
	/// by the server for cross-referencing resources inside batch/transaction
	/// requests.
	pub fn create<R: Into<V::Resource>>(&mut self, resource: R) -> String {
		let resource = resource.into();
		let uuid = format!("urn:uuid:{}", Uuid::new_v4());

		let entry = BundleEntry::<V>::empty()
			.with_full_url(uuid.clone())
			.with_request(BundleEntryRequest::<V>::make_post(
				resource.resource_type_str().to_owned(),
			))
			.with_resource(resource);

		self.entries.push(Some(entry));
		uuid
	}

	/// Add update of a resource to the batch/transaction.
	/// For advanced features like conditional creation, please use `with_raw`
	/// for now.
	pub fn update<R: Into<V::Resource>>(
		&mut self,
		resource: R,
		conditional: bool,
	) -> Result<(), Error> {
		let resource = resource.into();
		let resource_type = resource.resource_type_str();
		let resource_id = resource.id().ok_or(Error::MissingId)?;
		let full_url = self.client.url(&[resource_type, resource_id]);
		let url = format!("{resource_type}/{resource_id}");

		let mut request = BundleEntryRequest::<V>::make_put(url);
		if conditional {
			let version_id = resource.version_id().ok_or(Error::MissingVersionId)?;
			request = request.with_if_match(format!("W/\"{version_id}\""));
		}

		let entry = BundleEntry::<V>::empty()
			.with_full_url(full_url.to_string())
			.with_request(request)
			.with_resource(resource);

		self.entries.push(Some(entry));
		Ok(())
	}

	/// Add deletion of a resource to the batch/transaction.
	pub fn delete(&mut self, resource_type: V::ResourceType, id: &str) {
		let url = format!("{resource_type}/{id}");

		let entry =
			BundleEntry::<V>::empty().with_request(BundleEntryRequest::<V>::make_delete(url));

		self.entries.push(Some(entry));
	}

	/// Add retrieval of a resource to the batch/transaction.
	pub fn read(&mut self, resource_type: V::ResourceType, id: &str) {
		let url = format!("{resource_type}/{id}");

		let entry = BundleEntry::<V>::empty().with_request(BundleEntryRequest::<V>::make_get(url));

		self.entries.push(Some(entry));
	}

	/// Add a raw Bundle entry for more advanced queries.
	pub fn with_raw(&mut self, entry: BundleEntry<V>) {
		self.entries.push(Some(entry));
	}

	/// Send the batch or transaction to the server and receive the response.
	pub async fn send(self) -> Result<V::Bundle, Error> {
		let bundle = if self.is_transaction {
			V::Bundle::make_transaction(self.entries)
		} else {
			V::Bundle::make_batch(self.entries)
		};

		let url = self.client.url(&[]);
		let request = self
			.client
			.0
			.client
			.post(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(V::MIME_TYPE))
			.json(&bundle);

		let response = self.client.run_request(request).await?;
		if response.status().is_success() {
			let bundle: V::Bundle = response.json().await?;
			Ok(bundle)
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}
}

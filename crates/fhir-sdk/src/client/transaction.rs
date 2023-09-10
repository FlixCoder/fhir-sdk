//! Implementation of building batch/transaction requests and processing the
//! response.

use model::{
	codes::{BundleType, HTTPVerb},
	resources::{Bundle, BundleEntry, BundleEntryRequest, Resource, ResourceType},
};
use reqwest::header::{self, HeaderValue};
use uuid::Uuid;

use super::{model, Client, Error, MIME_TYPE};

/// A batch/transaction request builder.
#[derive(Debug, Clone)]
#[must_use = "You probably want to send the batch/transaction"]
pub struct BatchTransaction {
	/// The FHIR client.
	client: Client,
	/// If this is a transaction. Otherwise it is a batch request.
	is_transaction: bool,
	/// Current entries in the batch or transaction.
	entries: Vec<Option<BundleEntry>>,
}

impl BatchTransaction {
	/// Create new batch or transaction builder, given whether it is a
	/// transaction.
	pub fn new(client: Client, is_transaction: bool) -> Self {
		Self { client, is_transaction, entries: Vec::new() }
	}

	/// Add creation of a resource to the batch/transaction.
	/// For advanced features like conditional creation via `ifNoneExists`,
	/// please use `with_raw` for now.
	///
	/// Returns the temporary UUID to be used a reference that will be resolved
	/// by the server for cross-referencing resources inside batch/transaction
	/// requests.
	pub fn create(&mut self, resource: impl Into<Resource>) -> String {
		let resource = resource.into();
		let uuid = format!("urn:uuid:{}", Uuid::new_v4());

		let entry = BundleEntry::builder()
			.full_url(uuid.clone())
			.request(
				BundleEntryRequest::builder()
					.method(HTTPVerb::Post)
					.url(resource.resource_type().to_string())
					.build(),
			)
			.resource(resource)
			.build();

		self.entries.push(Some(entry));
		uuid
	}

	/// Add update of a resource to the batch/transaction.
	/// For advanced features like conditional creation, please use `with_raw`
	/// for now.
	pub fn update(
		&mut self,
		resource: impl Into<Resource>,
		conditional: bool,
	) -> Result<(), Error> {
		let resource = resource.into();
		let resource_type = resource.resource_type().as_str();
		let resource_id = resource.as_base_resource().id().as_ref().ok_or(Error::MissingId)?;
		let full_url = self.client.url(&[resource_type, resource_id]);
		let url = format!("{resource_type}/{resource_id}");

		let request = if conditional {
			let version_id = resource
				.as_base_resource()
				.meta()
				.as_ref()
				.and_then(|meta| meta.version_id.as_ref())
				.ok_or(Error::MissingVersionId)?;
			BundleEntryRequest::builder()
				.method(HTTPVerb::Put)
				.url(url)
				.if_match(format!("W/\"{version_id}\""))
				.build()
		} else {
			BundleEntryRequest::builder().method(HTTPVerb::Put).url(url).build()
		};

		let entry = BundleEntry::builder()
			.full_url(full_url.to_string())
			.request(request)
			.resource(resource)
			.build();

		self.entries.push(Some(entry));
		Ok(())
	}

	/// Add deletion of a resource to the batch/transaction.
	pub fn delete(&mut self, resource_type: ResourceType, id: &str) {
		let url = format!("{resource_type}/{id}");

		let entry = BundleEntry::builder()
			.request(BundleEntryRequest::builder().method(HTTPVerb::Delete).url(url).build())
			.build();

		self.entries.push(Some(entry));
	}

	/// Add retrieval of a resource to the batch/transaction.
	pub fn read(&mut self, resource_type: ResourceType, id: &str) {
		let url = format!("{resource_type}/{id}");

		let entry = BundleEntry::builder()
			.request(BundleEntryRequest::builder().method(HTTPVerb::Get).url(url).build())
			.build();

		self.entries.push(Some(entry));
	}

	/// Add a raw Bundle entry for more advanced queries.
	pub fn with_raw(&mut self, entry: BundleEntry) {
		self.entries.push(Some(entry));
	}

	/// Send the batch or transaction to the server and receive the response.
	pub async fn send(self) -> Result<Bundle, Error> {
		let bundle = Bundle::builder()
			.r#type(if self.is_transaction { BundleType::Transaction } else { BundleType::Batch })
			.entry(self.entries)
			.build();

		let url = self.client.url(&[]);
		let request = self
			.client
			.0
			.client
			.post(url)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE))
			.json(&bundle);

		let response = self.client.request_settings().make_request(request).await?;
		if response.status().is_success() {
			let bundle: Bundle = response.json().await?;
			Ok(bundle)
		} else {
			Err(Error::from_response(response).await)
		}
	}
}

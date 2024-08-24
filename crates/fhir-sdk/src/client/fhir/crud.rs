//! FHIR CRUD API interactions.

use fhir_model::{ParsedReference, WrongResourceType};
use reqwest::{
	header::{self, HeaderValue},
	StatusCode, Url,
};
use serde::{de::DeserializeOwned, Serialize};

use super::{
	misc,
	paging::Page,
	patch::{PatchViaFhir, PatchViaJson},
	transaction::BatchTransaction,
	Client, Error, SearchParameters,
};
use crate::{
	client::misc::{extract_header, make_uuid_header_value},
	extensions::{AnyResource, GenericResource, ReferenceExt},
	version::FhirVersion,
};

impl<V: FhirVersion> Client<V>
where
	(StatusCode, V::OperationOutcome): Into<Error>,
{
	/// Get the server's capabilities. Fails if the respective FHIR version is
	/// not supported at all.
	pub async fn capabilities(&self) -> Result<V::CapabilityStatement, Error> {
		let url = self.url(&["metadata"]);
		let request = self.0.client.get(url).header(header::ACCEPT, V::MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let capability_statement: V::CapabilityStatement = response.json().await?;
			Ok(capability_statement)
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Read any resource from any URL.
	pub(crate) async fn read_generic<R: DeserializeOwned>(
		&self,
		url: Url,
		correlation_id: Option<HeaderValue>,
	) -> Result<Option<R>, Error> {
		let mut request = self.0.client.get(url).header(header::ACCEPT, V::MIME_TYPE);
		if let Some(correlation_id) = correlation_id {
			request = request.header("X-Correlation-Id", correlation_id);
		}

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: R = response.json().await?;
			Ok(Some(resource))
		} else if [StatusCode::NOT_FOUND, StatusCode::GONE].contains(&response.status()) {
			Ok(None)
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Read the current version of a specific FHIR resource.
	pub async fn read<R: AnyResource<V> + DeserializeOwned>(
		&self,
		id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE_STR, id]);
		self.read_generic(url, None).await
	}

	/// Read a specific version of a specific FHIR resource.
	pub async fn read_version<R: AnyResource<V> + DeserializeOwned>(
		&self,
		id: &str,
		version_id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE_STR, id, "_history", version_id]);
		self.read_generic(url, None).await
	}

	/// Read the resource that is targeted in the reference.
	pub async fn read_referenced(&self, reference: &V::Reference) -> Result<V::Resource, Error> {
		let parsed_reference = reference.parse().ok_or(Error::MissingReference)?;
		let url = match parsed_reference {
			ParsedReference::Local { .. } => return Err(Error::LocalReference),
			ParsedReference::Relative { resource_type, id, version_id } => {
				if let Some(version_id) = version_id {
					self.url(&[resource_type, id, "_history", version_id])
				} else {
					self.url(&[resource_type, id])
				}
			}
			ParsedReference::Absolute { url, .. } => {
				url.parse().map_err(|_| Error::UrlParse(url.to_owned()))?
			}
		};

		let resource: V::Resource = self
			.read_generic(url.clone(), None)
			.await?
			.ok_or_else(|| Error::ResourceNotFound(url.to_string()))?;
		if let Some(resource_type) = reference.r#type() {
			if resource.resource_type_str() != resource_type {
				return Err(Error::WrongResourceType(
					resource.resource_type_str().to_owned(),
					resource_type.to_owned(),
				));
			}
		}

		Ok(resource)
	}

	/// Retrieve the history of the specified resource type or a specific resource.
	pub async fn history<R>(&self, id: Option<&str>) -> Result<Page<V, R>, Error>
	where
		R: AnyResource<V> + TryFrom<V::Resource, Error = WrongResourceType> + 'static,
		for<'a> &'a R: TryFrom<&'a V::Resource>,
	{
		let correlation_id = make_uuid_header_value();

		let url = {
			if let Some(id) = id {
				self.url(&[R::TYPE_STR, id, "_history"])
			} else {
				self.url(&[R::TYPE_STR, "_history"])
			}
		};
		let request = self
			.0
			.client
			.get(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header("X-Correlation-Id", correlation_id.clone());

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let bundle: V::Bundle = response.json().await?;
			Ok(Page::new(self.clone(), bundle, correlation_id))
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Inner function to create any resource for any resource type.
	pub(crate) async fn create_generic<R: Serialize + Send + Sync>(
		&self,
		resource_type: &str,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		let url = self.url(&[resource_type]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header(header::CONTENT_TYPE, V::MIME_TYPE)
			.json(resource);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let (id, version_id) = misc::parse_location(response.headers())?;
			let version_id = version_id.or(misc::parse_etag(response.headers()).ok());
			Ok((id, version_id))
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Create a new FHIR resource on the FHIR server. Returns the resource ID
	/// and version ID.
	pub async fn create<R: AnyResource<V> + Serialize + Send + Sync>(
		&self,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		self.create_generic(R::TYPE_STR, resource).await
	}

	/// Inner function to update any resource for any resource type.
	pub(crate) async fn update_generic<R: Serialize + Send + Sync>(
		&self,
		resource_type: &str,
		id: &str,
		resource: &R,
		version_id: Option<&str>,
	) -> Result<(bool, String), Error> {
		let url = self.url(&[resource_type, id]);
		let mut request = self
			.0
			.client
			.put(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header(header::CONTENT_TYPE, V::MIME_TYPE)
			.json(resource);
		if let Some(version_id) = version_id {
			let if_match = HeaderValue::from_str(&format!("W/\"{version_id}\""))
				.map_err(|_| Error::MissingVersionId)?;
			request = request.header(header::IF_MATCH, if_match);
		}

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let created = response.status() == StatusCode::CREATED;
			let version_id = misc::parse_etag(response.headers())?;
			Ok((created, version_id))
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Update a FHIR resource (or create it if it did not
	/// exist). If conditional update is selected, the resource is only updated
	/// if the version ID matches the expectations.
	pub async fn update<R: AnyResource<V> + Serialize + Send + Sync>(
		&self,
		resource: &R,
		conditional: bool,
	) -> Result<(bool, String), Error> {
		let id = resource.id().ok_or(Error::MissingId)?;
		let version_id = conditional
			.then(|| resource.version_id().ok_or(Error::MissingVersionId))
			.transpose()?;
		self.update_generic(R::TYPE_STR, id, resource, version_id).await
	}

	/// Delete a FHIR resource on the server.
	pub async fn delete(&self, resource_type: V::ResourceType, id: &str) -> Result<(), Error> {
		let url = self.url(&[resource_type.as_ref(), id]);
		let request = self.0.client.delete(url).header(header::ACCEPT, V::MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Search for FHIR resources of any type given the query parameters.
	pub async fn search_all(
		&self,
		queries: SearchParameters,
	) -> Result<Page<V, V::Resource>, Error> {
		// TODO: Use POST for long queries?

		let correlation_id = make_uuid_header_value();

		let url = self.url(&[]);
		let request = self
			.0
			.client
			.get(url)
			.query(&queries.into_queries())
			.header(header::ACCEPT, V::MIME_TYPE)
			.header("X-Correlation-Id", correlation_id.clone());

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let bundle: V::Bundle = response.json().await?;
			Ok(Page::new(self.clone(), bundle, correlation_id))
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Search for FHIR resources of a given type given the query parameters.
	pub async fn search<R>(&self, queries: SearchParameters) -> Result<Page<V, R>, Error>
	where
		R: AnyResource<V> + TryFrom<V::Resource, Error = WrongResourceType> + 'static,
		for<'a> &'a R: TryFrom<&'a V::Resource>,
	{
		// TODO: Use POST for long queries?

		let correlation_id = make_uuid_header_value();

		let url = self.url(&[R::TYPE_STR]);
		let request = self
			.0
			.client
			.get(url)
			.query(&queries.into_queries())
			.header(header::ACCEPT, V::MIME_TYPE)
			.header("X-Correlation-Id", correlation_id.clone());

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let bundle: V::Bundle = response.json().await?;
			Ok(Page::new(self.clone(), bundle, correlation_id))
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Search for FHIR resources via a custom request. This allows sending POST requests instead of
	/// GET when necessary. You can construct the request yourself to any URL and send any data.
	/// The endpoint is expected to send a FHIR-conform bundle.
	///
	/// You can specify the expected search entry type via the type parameter. This can be either
	/// the generic resource or a specific resource.
	///
	/// Keep in mind that mismatching origins to the base URL are rejected if not explicitly allowed
	/// via the flag in the builder ([ClientBuilder::allow_origin_mismatch]). Similarly, if the
	/// server responds with a different major FHIR version than the client is configured for, the
	/// response is rejected if not explicitly allowed via the flag in the builder
	/// ([ClientBuilder::allow_version_mismatch]).
	pub async fn search_custom<R>(
		&self,
		make_request: impl FnOnce(&reqwest::Client) -> reqwest::RequestBuilder + Send,
	) -> Result<Page<V, R>, Error>
	where
		R: TryFrom<V::Resource> + Send + Sync + 'static,
		for<'a> &'a R: TryFrom<&'a V::Resource>,
	{
		let request = (make_request)(&self.0.client);
		let (mut request, correlation_id) = extract_header(request, "X-Correlation-Id")?;
		let correlation_id = correlation_id.unwrap_or_else(make_uuid_header_value);
		request = request.header("X-Correlation-Id", correlation_id.clone());

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let bundle: V::Bundle = response.json().await?;
			Ok(Page::new(self.clone(), bundle, correlation_id))
		} else {
			Err(Error::from_response::<V>(response).await)
		}
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// `FHIRPath Patch` method.
	pub fn patch_via_fhir<'a>(
		&self,
		resource_type: V::ResourceType,
		id: &'a str,
	) -> PatchViaFhir<'a, V> {
		PatchViaFhir::new(self.clone(), resource_type, id)
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// [`JSON Patch`](https://datatracker.ietf.org/doc/html/rfc6902) method.
	pub fn patch_via_json<'a>(
		&self,
		resource_type: V::ResourceType,
		id: &'a str,
	) -> PatchViaJson<'a, V> {
		PatchViaJson::new(self.clone(), resource_type, id)
	}

	/// Start building a new batch request.
	pub fn batch(&self) -> BatchTransaction<V> {
		BatchTransaction::new(self.clone(), false)
	}

	/// Start building a new transaction request.
	pub fn transaction(&self) -> BatchTransaction<V> {
		BatchTransaction::new(self.clone(), true)
	}
}

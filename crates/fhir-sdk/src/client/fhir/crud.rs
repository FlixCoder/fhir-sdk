//! FHIR CRUD API interactions.

use fhir_model::{ParsedReference, WrongResourceType};
use futures::{Stream, TryStreamExt};
use reqwest::{
	header::{self, HeaderValue},
	StatusCode, Url,
};
use serde::{de::DeserializeOwned, Serialize};

use super::{misc, paging::Paged, Client, Error, SearchParameters};
use crate::{
	extensions::{AnyResource, BundleEntryExt, GenericResource, ReferenceExt, SearchEntryModeExt},
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
	) -> Result<Option<R>, Error> {
		let request = self.0.client.get(url).header(header::ACCEPT, V::MIME_TYPE);

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
		self.read_generic(url).await
	}

	/// Read a specific version of a specific FHIR resource.
	pub async fn read_version<R: AnyResource<V> + DeserializeOwned>(
		&self,
		id: &str,
		version_id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE_STR, id, "_history", version_id]);
		self.read_generic(url).await
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
			.read_generic(url.clone())
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

	// TODO: Refactor to improve:
	// - Do we want generic resource type instead of argument?
	// - We might need/want paging?
	/// Retrieve the history of a resource type or a specific resource.
	pub async fn history(
		&self,
		resource_type: V::ResourceType,
		id: Option<&str>,
	) -> Result<V::Bundle, Error> {
		let url = {
			if let Some(id) = id {
				self.url(&[resource_type.as_ref(), id, "_history"])
			} else {
				self.url(&[resource_type.as_ref(), "_history"])
			}
		};
		let request = self
			.0
			.client
			.get(url)
			.header(header::ACCEPT, V::MIME_TYPE)
			.header(header::CONTENT_TYPE, V::MIME_TYPE);
		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: V::Bundle = response.json().await?;
			Ok(resource)
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

	/// Search for any FHIR resources given the query parameters.
	pub fn search_all(
		&self,
		queries: SearchParameters,
	) -> impl Stream<Item = Result<V::Resource, Error>> + Send + 'static {
		let mut url = self.url(&[]);
		url.query_pairs_mut().extend_pairs(queries.into_queries()).finish();

		Paged::new(self.clone(), url, |entry| {
			entry.search_mode().map_or(true, |search_mode| search_mode.is_match())
		})
	}

	/// Search for FHIR resources of a given type given the query parameters.
	/// This simply ignores resources of the wrong type, e.g. an additional
	/// OperationOutcome.
	pub fn search<R: AnyResource<V> + TryFrom<V::Resource, Error = WrongResourceType>>(
		&self,
		queries: SearchParameters,
	) -> impl Stream<Item = Result<R, Error>> + Send + 'static {
		let mut url = self.url(&[R::TYPE_STR]);
		url.query_pairs_mut().extend_pairs(queries.into_queries()).finish();

		Paged::new(self.clone(), url, |entry| {
			entry.search_mode().map_or(true, |search_mode| search_mode.is_match())
		})
		.try_filter_map(|resource| async move { Ok(R::try_from(resource).ok()) })
	}
}

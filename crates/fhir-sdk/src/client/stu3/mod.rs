//! FHIR STU3 client implementation.

mod paging;
mod patch;
mod search;
mod transaction;

use fhir_model::{
	stu3::{
		codes::SearchEntryMode,
		resources::{
			BaseResource, Bundle, CapabilityStatement, NamedResource, Parameters,
			ParametersParameter, ParametersParameterValue, Patient, Resource, ResourceType,
		},
		types::Reference,
		JSON_MIME_TYPE,
	},
	ParsedReference, WrongResourceType,
};
use futures::{Stream, TryStreamExt};
use reqwest::{
	header::{self, HeaderValue},
	StatusCode, Url,
};
use serde::{de::DeserializeOwned, Serialize};

pub use self::search::{
	DateSearch, MissingSearch, NumberSearch, QuantitySearch, ReferenceSearch, StringSearch,
	TokenSearch, UriSearch,
};
use self::{
	paging::Paged,
	patch::{PatchViaFhir, PatchViaJson},
	transaction::BatchTransaction,
};
use super::{misc, Client, Error, FhirStu3, SearchParameters};

/// FHIR MIME-type this client uses.
const MIME_TYPE: &str = JSON_MIME_TYPE;

impl Client<FhirStu3> {
	/// Get the server's capabilities. Fails if the respective FHIR version is
	/// not supported at all.
	pub async fn capabilities(&self) -> Result<CapabilityStatement, Error> {
		let url = self.url(&["metadata"]);
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let capability_statement: CapabilityStatement = response.json().await?;
			Ok(capability_statement)
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Read any resource from any URL.
	async fn read_generic<R: DeserializeOwned>(&self, url: Url) -> Result<Option<R>, Error> {
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: R = response.json().await?;
			Ok(Some(resource))
		} else if [StatusCode::NOT_FOUND, StatusCode::GONE].contains(&response.status()) {
			Ok(None)
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Read the current version of a specific FHIR resource.
	pub async fn read<R: NamedResource + DeserializeOwned>(
		&self,
		id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE.as_str(), id]);
		self.read_generic(url).await
	}

	/// Read a specific version of a specific FHIR resource.
	pub async fn read_version<R: NamedResource + DeserializeOwned>(
		&self,
		id: &str,
		version_id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE.as_str(), id, "_history", version_id]);
		self.read_generic(url).await
	}

	/// Read the resource that is targeted in the reference.
	pub async fn read_referenced(&self, reference: &Reference) -> Result<Resource, Error> {
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

		let resource: Resource = self
			.read_generic(url.clone())
			.await?
			.ok_or_else(|| Error::ResourceNotFound(url.to_string()))?;

		Ok(resource)
	}

	/// Inner function to create any resource for any resource type.
	pub(crate) async fn create_generic<R: Serialize + Send + Sync>(
		&self,
		resource_type: ResourceType,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		let url = self.url(&[resource_type.as_str()]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE)
			.json(resource);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let (id, version_id) = misc::parse_location(response.headers())?;
			let version_id = version_id.or(misc::parse_etag(response.headers()).ok());
			Ok((id, version_id))
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Create a new FHIR resource on the FHIR server. Returns the resource ID
	/// and version ID.
	pub async fn create<R: NamedResource + Serialize + Send + Sync>(
		&self,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		self.create_generic(R::TYPE, resource).await
	}

	/// Inner function to update any resource for any resource type.
	pub(crate) async fn update_generic<R: Serialize + Send + Sync>(
		&self,
		resource_type: ResourceType,
		id: &str,
		resource: &R,
		version_id: Option<&str>,
	) -> Result<(bool, String), Error> {
		let url = self.url(&[resource_type.as_str(), id]);
		let mut request = self
			.0
			.client
			.put(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE)
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
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Update a FHIR resource (or create it if it did not
	/// exist). If conditional update is selected, the resource is only updated
	/// if the version ID matches the expectations.
	pub async fn update<R: NamedResource + BaseResource + Serialize + Send + Sync>(
		&self,
		resource: &R,
		conditional: bool,
	) -> Result<(bool, String), Error> {
		let id = resource.id().as_ref().ok_or(Error::MissingId)?;
		let version_id = conditional
			.then(|| {
				resource
					.meta()
					.as_ref()
					.and_then(|meta| meta.version_id.as_deref())
					.ok_or(Error::MissingVersionId)
			})
			.transpose()?;
		self.update_generic(R::TYPE, id, resource, version_id).await
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// `FHIRPath Patch` method.
	pub fn patch_via_fhir<'a>(&self, resource_type: ResourceType, id: &'a str) -> PatchViaFhir<'a> {
		PatchViaFhir::new(self.clone(), resource_type, id)
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// [`JSON Patch`](https://datatracker.ietf.org/doc/html/rfc6902) method.
	pub fn patch_via_json<'a>(&self, resource_type: ResourceType, id: &'a str) -> PatchViaJson<'a> {
		PatchViaJson::new(self.clone(), resource_type, id)
	}

	/// Delete a FHIR resource on the server.
	pub async fn delete(&self, resource_type: ResourceType, id: &str) -> Result<(), Error> {
		let url = self.url(&[resource_type.as_str(), id]);
		let request = self.0.client.delete(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Search for any FHIR resources given the query parameters.
	pub fn search_all(
		&self,
		queries: SearchParameters,
	) -> impl Stream<Item = Result<Resource, Error>> + Send + 'static {
		let mut url = self.url(&[]);
		url.query_pairs_mut().extend_pairs(queries.into_queries()).finish();

		Paged::new(self.clone(), url, |entry| {
			entry
				.search
				.as_ref()
				.and_then(|search| search.mode.as_ref())
				.map_or(true, |search_mode| *search_mode == SearchEntryMode::Match)
		})
	}

	/// Search for FHIR resources of a given type given the query parameters.
	/// This simply ignores resources of the wrong type, e.g. an additional
	/// OperationOutcome.
	pub fn search<R: NamedResource + TryFrom<Resource, Error = WrongResourceType>>(
		&self,
		queries: SearchParameters,
	) -> impl Stream<Item = Result<R, Error>> + Send + 'static {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(queries.into_queries()).finish();

		Paged::new(self.clone(), url, |entry| {
			entry
				.search
				.as_ref()
				.and_then(|search| search.mode.as_ref())
				.map_or(true, |search_mode| *search_mode == SearchEntryMode::Match)
		})
		.try_filter_map(|resource| async move { Ok(R::try_from(resource).ok()) })
	}

	/// Start building a new batch request.
	pub fn batch(&self) -> BatchTransaction {
		BatchTransaction::new(self.clone(), false)
	}

	/// Start building a new transaction request.
	pub fn transaction(&self) -> BatchTransaction {
		BatchTransaction::new(self.clone(), true)
	}

	/// Operation `$everything` on `Encounter`, returning a Bundle with all
	/// resources for an `Encounter` record.
	pub async fn operation_encounter_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Encounter", id, "$everything"]);
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Operation `$everything` on `Patient`, returning a Bundle with all
	/// resources for an `Patient` record.
	pub async fn operation_patient_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Patient", id, "$everything"]);
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Operation `$match` on `Patient`, returning matches for Patient records
	/// based on a given incomplete Patient resource.
	pub async fn operation_patient_match(
		&self,
		patient: Patient,
		only_certain: bool,
		count: i32,
	) -> Result<Bundle, Error> {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameters = Parameters::builder()
			.parameter(vec![
				Some(
					ParametersParameter::builder()
						.name("resource".to_owned())
						.resource(Resource::from(patient))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("onlyCertainMatches".to_owned())
						.value(ParametersParameterValue::Boolean(only_certain))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("count".to_owned())
						.value(ParametersParameterValue::Integer(count))
						.build()
						.unwrap(),
				),
			])
			.build()
			.unwrap();

		let url = self.url(&["Patient", "$match"]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE)
			.json(&parameters);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}

	/// Retrieve the history of a resource type or a specific resource.
	pub async fn history(
		&self,
		resource_type: ResourceType,
		id: Option<&str>,
	) -> Result<Bundle, Error> {
		let url = {
			if let Some(id) = id {
				self.url(&[resource_type.as_str(), id, "_history"])
			} else {
				self.url(&[resource_type.as_str(), "_history"])
			}
		};
		let request = self
			.0
			.client
			.get(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE);
		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response_stu3(response).await)
		}
	}
}

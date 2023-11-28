//! REST Client Implementation.
//!
//! Does only work with one FHIR version at a time!

mod builder;
mod error;
mod misc;
mod paging;
mod patch;
mod request;
mod search;
mod transaction;
mod write;

use std::{
	future::Future,
	pin::Pin,
	sync::{Arc, Mutex},
};

#[cfg(feature = "r4b")]
use fhir_model::r4b as model;
#[cfg(feature = "r5")]
use fhir_model::r5 as model;
use fhir_model::ParsedReference;
use futures::stream::{Stream, TryStreamExt};
use model::{
	codes::SearchEntryMode,
	resources::{
		BaseResource, Bundle, CapabilityStatement, NamedResource, Parameters, ParametersParameter,
		ParametersParameterValue, Patient, Resource, ResourceType, WrongResourceType,
	},
	types::Reference,
	JSON_MIME_TYPE,
};
#[cfg(feature = "r5")]
use model::{codes::SubscriptionPayloadContent, resources::SubscriptionStatus};
pub use reqwest::{
	header::{self, HeaderMap, HeaderValue},
	StatusCode, Url,
};
use serde::{de::DeserializeOwned, Serialize};

pub use self::{
	builder::ClientBuilder,
	error::Error,
	request::RequestSettings,
	search::{
		DateSearch, NumberSearch, QuantitySearch, ReferenceSearch, SearchParameter,
		SearchParameters, StringSearch, TokenSearch, UriSearch,
	},
	write::ResourceWrite,
};
use self::{
	paging::Paged,
	patch::{PatchViaFhir, PatchViaJson},
	transaction::BatchTransaction,
};

/// User agent of this client.
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
/// FHIR MIME-type this client uses.
const MIME_TYPE: &str = JSON_MIME_TYPE;

/// FHIR REST Client.
#[derive(Debug, Clone)]
pub struct Client(Arc<ClientData>);

/// Return type of the auth callback.
type AuthCallbackReturn = Result<HeaderValue, Box<dyn std::error::Error + Send + Sync>>;
/// Auth callback function type.
type AuthCallback =
	Arc<dyn Fn() -> Pin<Box<dyn Future<Output = AuthCallbackReturn> + Send>> + Send + Sync>;

/// FHIR Rest Client data.
struct ClientData {
	/// The FHIR server's base URL.
	base_url: Url,
	/// HTTP request client.
	client: reqwest::Client,
	/// Request settings.
	request_settings: Mutex<RequestSettings>,
	/// Authorization callback method, returning the authorization header value.
	auth_callback: Mutex<Option<AuthCallback>>,
}

impl Client {
	/// Start building a new client with custom settings.
	#[must_use]
	pub fn builder() -> ClientBuilder {
		ClientBuilder::default()
	}

	/// Create a new client with default settings.
	pub fn new(base_url: Url) -> Result<Self, Error> {
		if base_url.cannot_be_a_base() {
			return Err(Error::UrlCannotBeBase);
		}

		let mut headers = HeaderMap::new();
		headers.insert(header::ACCEPT, HeaderValue::from_static(MIME_TYPE));
		headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE));
		let client =
			reqwest::Client::builder().user_agent(USER_AGENT).default_headers(headers).build()?;

		let data = ClientData {
			base_url,
			client,
			request_settings: Mutex::new(RequestSettings::default()),
			auth_callback: Mutex::new(None),
		};
		Ok(Self(Arc::new(data)))
	}

	/// Get the request settings configured in this client.
	#[must_use]
	pub fn request_settings(&self) -> RequestSettings {
		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		self.0.request_settings.lock().expect("mutex poisened").clone()
	}

	/// Set the request settings for this client.
	pub fn set_request_settings(&self, settings: RequestSettings) {
		tracing::debug!("Setting new request settings");
		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		let mut request_settings = self.0.request_settings.lock().expect("mutex poisened");
		*request_settings = settings;
	}

	/// Get the auth callback configured in this client.
	#[must_use]
	pub fn auth_callback(&self) -> Option<AuthCallback> {
		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		self.0.auth_callback.lock().expect("mutex poisened").clone()
	}

	/// Run a request using the internal request settings, calling the auth
	/// callback to retrieve a new Authorization header on `unauthtorized`
	/// responses.
	async fn run_request(
		&self,
		request: reqwest::RequestBuilder,
	) -> Result<reqwest::Response, Error> {
		// Try running the request
		let mut request_settings = self.request_settings();
		let response = request_settings
			.make_request(request.try_clone().ok_or(Error::RequestNotClone)?)
			.await?;

		// On authorization failure, retry after refreshing the authorization header.
		if response.status() == StatusCode::UNAUTHORIZED {
			if let Some(auth_callback) = self.auth_callback() {
				tracing::info!("Hit unauthorized response, calling auth_callback");
				let auth_value = (auth_callback)()
					.await
					.map_err(|err| Error::AuthCallback(format!("{err:#}")))?;
				request_settings =
					request_settings.header(reqwest::header::AUTHORIZATION, auth_value);
				self.set_request_settings(request_settings.clone());
				let response = request_settings.make_request(request).await?;
				return Ok(response);
			}
		}

		Ok(response)
	}

	/// Get the URL with the configured base URL and the given path segments.
	fn url(&self, segments: &[&str]) -> Url {
		let mut url = self.0.base_url.clone();
		#[allow(clippy::expect_used)] // We made sure of it in the constructor.
		url.path_segments_mut().expect("Base URL cannot be base").pop_if_empty().extend(segments);
		url
	}

	/// Get the server's capabilities. Fails if the respective FHIR version is
	/// not supported at all.
	pub async fn capabilities(&self) -> Result<CapabilityStatement, Error> {
		let url = self.url(&["metadata"]);
		let request = self.0.client.get(url);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let capability_statement: CapabilityStatement = response.json().await?;
			Ok(capability_statement)
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Read any resource from any URL.
	async fn read_generic<R: DeserializeOwned>(&self, url: Url) -> Result<Option<R>, Error> {
		let request = self.0.client.get(url);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: R = response.json().await?;
			Ok(Some(resource))
		} else if [StatusCode::NOT_FOUND, StatusCode::GONE].contains(&response.status()) {
			Ok(None)
		} else {
			Err(Error::from_response(response).await)
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
		if let Some(resource_type) = reference.r#type.as_ref() {
			if resource.resource_type().as_str() != resource_type {
				return Err(Error::WrongResourceType(
					resource.resource_type().to_string(),
					resource_type.clone(),
				));
			}
		}

		Ok(resource)
	}

	/// Create a new FHIR resource on the FHIR server. Returns the resource ID
	/// and version ID.
	pub async fn create<R: NamedResource + Serialize + Send + Sync>(
		&self,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		let url = self.url(&[R::TYPE.as_str()]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE))
			.json(resource);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let (id, version_id) = misc::parse_location(response.headers())?;
			let version_id = version_id.or(misc::parse_etag(response.headers()).ok());
			Ok((id, version_id))
		} else {
			Err(Error::from_response(response).await)
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

		let url = self.url(&[R::TYPE.as_str(), id]);
		let mut request = self
			.0
			.client
			.put(url)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE))
			.json(resource);
		if conditional {
			let version_id = resource
				.meta()
				.as_ref()
				.and_then(|meta| meta.version_id.as_ref())
				.ok_or(Error::MissingVersionId)?;
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
			Err(Error::from_response(response).await)
		}
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
		let request = self.0.client.delete(url);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response(response).await)
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
		let request = self.0.client.get(url);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Operation `$everything` on `Patient`, returning a Bundle with all
	/// resources for an `Patient` record.
	pub async fn operation_patient_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Patient", id, "$everything"]);
		let request = self.0.client.get(url);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response(response).await)
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
		let parameters = Parameters::builder()
			.parameter(vec![
				Some(
					ParametersParameter::builder()
						.name("resource".to_owned())
						.resource(Resource::from(patient))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("onlyCertainMatches".to_owned())
						.value(ParametersParameterValue::Boolean(only_certain))
						.build(),
				),
				Some(
					ParametersParameter::builder()
						.name("count".to_owned())
						.value(ParametersParameterValue::Integer(count))
						.build(),
				),
			])
			.build();

		let url = self.url(&["Patient", "$match"]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::CONTENT_TYPE, HeaderValue::from_static(MIME_TYPE))
			.json(&parameters);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let resource: Bundle = response.json().await?;
			Ok(resource)
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Operation `$status` on `Subscription`, returning the
	/// `SubcriptionStatus`.
	#[cfg(feature = "r5")]
	pub async fn operation_subscription_status(
		&self,
		id: &str,
	) -> Result<SubscriptionStatus, Error> {
		let url = self.url(&["Subscription", id, "$status"]);
		let request = self.0.client.get(url.clone());

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let bundle: Bundle = response.json().await?;
			let resource = bundle
				.0
				.entry
				.into_iter()
				.flatten()
				.filter_map(|entry| entry.resource)
				.find_map(|res| SubscriptionStatus::try_from(res).ok())
				.ok_or_else(|| Error::ResourceNotFound(url.to_string()))?;
			Ok(resource)
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Operation `$events` on `Subscription`, returning the previous
	/// notifications that were triggered by a topic.
	#[cfg(feature = "r5")]
	pub async fn operation_subscription_events(
		&self,
		id: &str,
		events_since: Option<i64>,
		events_until: Option<i64>,
		content: Option<SubscriptionPayloadContent>,
	) -> Result<Bundle, Error> {
		let mut queries = Vec::new();
		if let Some(events_since) = events_since {
			queries.push(("eventsSinceNumber", events_since.to_string()));
		}
		if let Some(events_until) = events_until {
			queries.push(("eventsUntilNumber", events_until.to_string()));
		}
		if let Some(content) = content {
			queries.push(("content", content.to_string()));
		}

		let url = self.url(&["Subscription", id, "$events"]);
		let request = self.0.client.get(url).query(&queries);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let bundle: Bundle = response.json().await?;
			Ok(bundle)
		} else {
			Err(Error::from_response(response).await)
		}
	}
}

impl std::fmt::Debug for ClientData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let auth_callback = match self.auth_callback.try_lock() {
			Ok(inside) => {
				if inside.is_some() {
					"Some(<fn>)"
				} else {
					"None"
				}
			}
			Err(_) => "<blocked>",
		};

		f.debug_struct("ClientData")
			.field("base_url", &self.base_url)
			.field("client", &self.client)
			.field("request_settings", &self.request_settings)
			.field("auth_callback", &auth_callback)
			.finish()
	}
}

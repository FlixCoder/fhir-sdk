//! REST Client Implementation.
//!
//! Does only work with one FHIR version at a time!

mod error;
mod paging;
mod request;
mod write;

use std::sync::{Arc, Mutex};

#[cfg(feature = "r4b")]
use fhir_model::r4b as model;
#[cfg(feature = "r5")]
use fhir_model::r5 as model;
use futures::{Stream, TryStreamExt};
use model::{
	resources::{BaseResource, NamedResource, Resource, ResourceType, WrongResourceType},
	JSON_MIME_TYPE,
};
use reqwest::{
	header::{self, HeaderMap, HeaderValue},
	StatusCode, Url,
};
use serde::{de::DeserializeOwned, Serialize};

pub use self::{error::Error, paging::Paged, request::RequestSettings, write::ResourceWrite};

/// User agent of this client.
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
/// FHIR MIME-type this client uses.
const MIME_TYPE: &str = JSON_MIME_TYPE;

/// FHIR REST Client.
#[derive(Debug, Clone)]
pub struct Client(Arc<ClientData>);

/// FHIR Rest Client data.
#[derive(Debug)]
struct ClientData {
	/// The FHIR server's base URL.
	base_url: Url,
	/// HTTP request client.
	client: reqwest::Client,
	/// Request settings.
	request_settings: Mutex<RequestSettings>,
}

impl Client {
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
		#![allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		*self.0.request_settings.lock().expect("mutex poisened") = settings;
	}

	/// Get the URL with the configured base URL and the given path segments.
	fn url(&self, segments: &[&str]) -> Url {
		let mut url = self.0.base_url.clone();
		#[allow(clippy::expect_used)] // We made sure of it in the constructor.
		url.path_segments_mut().expect("Base URL cannot be base").pop_if_empty().extend(segments);
		url
	}

	/// Read the current version of a specific FHIR resource.
	pub async fn read<R: NamedResource + DeserializeOwned>(
		&self,
		id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE.as_str(), id]);
		let request = self.0.client.get(url);

		let response = self.request_settings().make_request(request).await?;
		if response.status().is_success() {
			let resource: R = response.json().await?;
			Ok(Some(resource))
		} else if [StatusCode::NOT_FOUND, StatusCode::GONE].contains(&response.status()) {
			Ok(None)
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Read a specific version of a specific FHIR resource.
	pub async fn read_version<R: NamedResource + DeserializeOwned>(
		&self,
		id: &str,
		version_id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE.as_str(), id, "_history", version_id]);
		let request = self.0.client.get(url);

		let response = self.request_settings().make_request(request).await?;
		if response.status().is_success() {
			let resource: R = response.json().await?;
			Ok(Some(resource))
		} else if [StatusCode::NOT_FOUND, StatusCode::GONE].contains(&response.status()) {
			Ok(None)
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
		let mut request = self.0.client.put(url).json(resource);
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

		let response = self.request_settings().make_request(request).await?;
		if response.status().is_success() {
			let created = response.status() == StatusCode::CREATED;
			let version_id = parse_etag(response.headers())?;
			Ok((created, version_id))
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Create a new FHIR resource on the FHIR server. Returns the resource ID
	/// and version ID.
	pub async fn create<R: NamedResource + Serialize + Send + Sync>(
		&self,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		let url = self.url(&[R::TYPE.as_str()]);
		let request = self.0.client.post(url).json(resource);

		let response = self.request_settings().make_request(request).await?;
		if response.status().is_success() {
			let (id, version_id) = parse_location(response.headers())?;
			let version_id = version_id.or(parse_etag(response.headers()).ok());
			Ok((id, version_id))
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Delete a FHIR resource on the server.
	pub async fn delete(&self, resource_type: ResourceType, id: &str) -> Result<(), Error> {
		let url = self.url(&[resource_type.as_str(), id]);
		let request = self.0.client.delete(url);

		let response = self.request_settings().make_request(request).await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(Error::from_response(response).await)
		}
	}

	/// Search for FHIR resources of a given type given the raw query
	/// parameters. This simply ignores resources of the wrong type, e.g. an
	/// additional OperationOutcome.
	pub fn search_raw<R: NamedResource + TryFrom<Resource, Error = WrongResourceType>>(
		&self,
		queries: &[(&str, &str)],
	) -> impl Stream<Item = Result<R, Error>> {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(queries).finish();

		Paged::new(self.clone(), url)
			.try_filter_map(|resource| async move { Ok(R::try_from(resource).ok()) })
	}
}

/// Parse an ETag to a version ID.
fn parse_etag(headers: &HeaderMap) -> Result<String, Error> {
	let etag = headers
		.get(header::ETAG)
		.ok_or(Error::EtagFailure)?
		.to_str()
		.map_err(|_| Error::EtagFailure)?;
	if etag.starts_with("W/\"") && etag.ends_with('"') {
		let end = etag.split_at(3).1;
		let version_id = end.split_at(end.len() - 1).0;
		Ok(version_id.to_owned())
	} else {
		Err(Error::EtagFailure)
	}
}

/// Parse an Location header to a resource ID and optional version ID.
fn parse_location(headers: &HeaderMap) -> Result<(String, Option<String>), Error> {
	let location = headers
		.get(header::LOCATION)
		.ok_or(Error::LocationFailure)?
		.to_str()
		.map_err(|_| Error::LocationFailure)?;
	let mut segments = location.rsplit('/');
	let id_or_version_id = segments.next().ok_or(Error::LocationFailure)?;
	let history_or_type = segments.next().ok_or(Error::LocationFailure)?;

	if history_or_type == "_history" {
		let id = segments.next().ok_or(Error::LocationFailure)?;
		Ok((id.to_owned(), Some(id_or_version_id.to_owned())))
	} else {
		Ok((id_or_version_id.to_owned(), None))
	}
}

#[cfg(test)]
mod tests;

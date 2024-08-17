//! Client errors.

#[cfg(feature = "r4b")]
use fhir_model::r4b;
#[cfg(feature = "r5")]
use fhir_model::r5;
#[cfg(feature = "stu3")]
use fhir_model::stu3;
use reqwest::StatusCode;
use thiserror::Error;

use crate::version::FhirVersion;

/// FHIR REST Client Error.
#[derive(Debug, Error)]
pub enum Error {
	/// Builder is missing a field to construct the client.
	#[error("Builder is missing field `{0}` to construct the client")]
	BuilderMissingField(&'static str),

	/// URL cannot be a base URL.
	#[error("Given base URL cannot be a base URL")]
	UrlCannotBeBase,

	/// Failed parsing an URL.
	#[error("Failed parsing the URL: {0}")]
	UrlParse(String),

	/// Missing resource ID.
	#[error("Resource is missing ID")]
	MissingId,

	/// Missing resource version ID.
	#[error("Resource is missing version ID")]
	MissingVersionId,

	/// Missing reference field in FHIR reference.
	#[error("Missing reference URL in reference")]
	MissingReference,

	/// Reference was to local resource.
	#[error("Tried to fetch local reference")]
	LocalReference,

	/// Request was not clonable.
	#[error("Was not able to clone HTTP Request")]
	RequestNotClone,

	/// Found URL with unexpected origin.
	#[error("URLs with mismatching origins are disabled: {0}")]
	DifferentOrigin(String),

	/// Auth callback error.
	#[error("Authorization callback error: {0}")]
	AuthCallback(String),

	/// Serialization/Deserialization error.
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),

	/// HTTP Request error.
	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),

	/// HTTP error response.
	#[error("Got error response ({0}): {1}")]
	Response(StatusCode, String),

	/// Mismatching FHIR version in response.
	#[error("Server responded with mismatching major FHIR version: {0}")]
	DifferentFhirVersion(String),

	#[cfg(feature = "stu3")]
	/// OperationOutcome.
	#[error("OperationOutcome({0}): {1:?}")]
	OperationOutcomeStu3(StatusCode, stu3::resources::OperationOutcome),

	#[cfg(feature = "r4b")]
	/// OperationOutcome.
	#[error("OperationOutcome({0}): {1:?}")]
	OperationOutcomeR4B(StatusCode, r4b::resources::OperationOutcome),

	#[cfg(feature = "r5")]
	/// OperationOutcome.
	#[error("OperationOutcome({0}): {1:?}")]
	OperationOutcomeR5(StatusCode, r5::resources::OperationOutcome),

	/// Resource was not found.
	#[error("Resource `{0}` was not found")]
	ResourceNotFound(String),

	/// Failed to parse header value due to non-ASCII value content.
	#[error("Failed to parse header value: {0}")]
	HeaderParsing(#[from] reqwest::header::ToStrError),

	/// Error parsing ETag to version ID, i.e. missing ETag or wrong format.
	#[error("Missing or wrong ETag in response: {0}")]
	EtagFailure(String),

	/// Response did not provide `Location` header or it failed to parse.
	#[error("Missing or wrong Location header in response: {0}")]
	LocationFailure(String),

	/// Wrong resource was delivered.
	#[error("Resource type {0} is not the requested type {1}")]
	WrongResourceType(String, String),
}

impl Error {
	/// Whether the error should likely be retried.
	#[must_use]
	pub fn should_retry(&self) -> bool {
		tracing::debug!("Checking if error `{self}` should be retried");
		match self {
			Self::Request(err) => err.is_connect() || err.is_request() || err.is_timeout(),
			_ => false,
		}
	}

	/// Extract the error from a response.
	pub(crate) async fn from_response<V>(response: reqwest::Response) -> Self
	where
		V: FhirVersion,
		(StatusCode, V::OperationOutcome): Into<Self>,
	{
		let status = response.status();
		let body = response.text().await.unwrap_or_default();
		if let Ok(outcome) = serde_json::from_str::<V::OperationOutcome>(&body) {
			(status, outcome).into()
		} else {
			Self::Response(status, body)
		}
	}
}

#[cfg(feature = "stu3")]
impl From<(StatusCode, stu3::resources::OperationOutcome)> for Error {
	fn from((status, outcome): (StatusCode, stu3::resources::OperationOutcome)) -> Self {
		Self::OperationOutcomeStu3(status, outcome)
	}
}

#[cfg(feature = "r4b")]
impl From<(StatusCode, r4b::resources::OperationOutcome)> for Error {
	fn from((status, outcome): (StatusCode, r4b::resources::OperationOutcome)) -> Self {
		Self::OperationOutcomeR4B(status, outcome)
	}
}

#[cfg(feature = "r5")]
impl From<(StatusCode, r5::resources::OperationOutcome)> for Error {
	fn from((status, outcome): (StatusCode, r5::resources::OperationOutcome)) -> Self {
		Self::OperationOutcomeR5(status, outcome)
	}
}

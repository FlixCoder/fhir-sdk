//! Miscellaneous helpers.

use ::uuid::Uuid;
use reqwest::header::{self, HeaderMap, HeaderValue};

use super::Error;

/// Parse an ETag to a version ID.
pub fn parse_etag(headers: &HeaderMap) -> Result<String, Error> {
	let etag =
		headers.get(header::ETAG).ok_or_else(|| Error::EtagFailure("None".to_owned()))?.to_str()?;
	if etag.starts_with("W/\"") && etag.ends_with('"') {
		let end = etag.split_at(3).1;
		let version_id = end.split_at(end.len() - 1).0;
		Ok(version_id.to_owned())
	} else if etag.starts_with('"') && etag.ends_with('"') {
		// Spec says one should use weak etags `W/"version"`, but we can work with it if
		// the server does not do this.
		let end = etag.split_at(1).1;
		let version_id = end.split_at(end.len() - 1).0;
		Ok(version_id.to_owned())
	} else {
		Err(Error::EtagFailure(etag.to_owned()))
	}
}

/// Parse an Location header to a resource ID and optional version ID.
pub fn parse_location(headers: &HeaderMap) -> Result<(String, Option<String>), Error> {
	let location = headers
		.get(header::LOCATION)
		.ok_or_else(|| Error::LocationFailure("None".to_owned()))?
		.to_str()?;
	let mut segments = location.rsplit('/');
	let id_or_version_id =
		segments.next().ok_or_else(|| Error::LocationFailure(location.to_owned()))?;
	let history_or_type =
		segments.next().ok_or_else(|| Error::LocationFailure(location.to_owned()))?;

	if history_or_type == "_history" {
		let id = segments.next().ok_or_else(|| Error::LocationFailure(location.to_owned()))?;
		Ok((id.to_owned(), Some(id_or_version_id.to_owned())))
	} else {
		Ok((id_or_version_id.to_owned(), None))
	}
}

/// Parse major FHIR version from Content-Type header, if it exists.
pub fn parse_major_fhir_version(headers: &HeaderMap) -> Result<Option<&str>, Error> {
	let Some(header) = headers.get(header::CONTENT_TYPE) else {
		return Ok(None);
	};
	let header_str = header.to_str()?;
	let version = header_str
		.split(';')
		.filter_map(|param| param.split_once('='))
		.find_map(|(key, value)| (key.trim().to_lowercase() == "fhirversion").then_some(value))
		.map(|version| version.split_once('.').unwrap_or((version, "")).0);
	Ok(version)
}

/// Escape a search parameter value.
pub fn escape_search_value(value: &str) -> String {
	value.replace('\\', "\\\\").replace('|', "\\|").replace('$', "\\$").replace(',', "\\,")
}

/// Make a [HeaderValue] containing a new UUID.
pub fn make_uuid_header_value() -> HeaderValue {
	#[allow(clippy::expect_used, reason = "UUIDs are valid header values")]
	HeaderValue::from_str(&Uuid::new_v4().to_string()).expect("UUIDs are valid header values")
}

#[cfg(test)]
mod tests {
	#![allow(clippy::expect_used, reason = "Allowed for tests")]

	use reqwest::header::HeaderValue;

	use super::*;

	#[test]
	fn etag_parsing() {
		let mut headers = HeaderMap::new();

		headers.insert(header::ETAG, HeaderValue::from_static("W/\"1\""));
		let version_id = parse_etag(&headers).expect("parsing ETag");
		assert_eq!(version_id, "1");

		headers.insert(header::ETAG, HeaderValue::from_static("W/\"W/\"1\"\""));
		let version_id = parse_etag(&headers).expect("parsing ETag");
		assert_eq!(version_id, "W/\"1\"");

		headers.insert(header::ETAG, HeaderValue::from_static("1"));
		let result = parse_etag(&headers);
		assert!(matches!(result, Err(Error::EtagFailure(_))));
	}

	#[test]
	fn location_parsing() {
		let mut headers = HeaderMap::new();

		headers.insert(header::LOCATION, HeaderValue::from_static("/Patient/123/_history/1"));
		let (id, version_id) = parse_location(&headers).expect("parsing Location");
		assert_eq!(id, "123");
		assert_eq!(version_id.as_deref(), Some("1"));

		headers.insert(header::LOCATION, HeaderValue::from_static("Encounter/123"));
		let (id, version_id) = parse_location(&headers).expect("parsing Location");
		assert_eq!(id, "123");
		assert_eq!(version_id, None);

		headers.insert(
			header::LOCATION,
			HeaderValue::from_static("/something/base/Patient/123/_history/1"),
		);
		let (id, version_id) = parse_location(&headers).expect("parsing Location");
		assert_eq!(id, "123");
		assert_eq!(version_id.as_deref(), Some("1"));
	}

	#[test]
	fn fhir_version_parsing() {
		let mut headers = HeaderMap::new();

		headers.insert(
			header::CONTENT_TYPE,
			HeaderValue::from_static("application/fhir+json; fhirVersion=4.0"),
		);
		let version = parse_major_fhir_version(&headers).expect("parsing FHIR version");
		assert_eq!(version, Some("4"));
	}
}

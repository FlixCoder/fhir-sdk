//! Basic unit tests for the client.
#![allow(clippy::expect_used)] // Allowed for tests

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
	assert!(matches!(result, Err(Error::EtagFailure)));
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

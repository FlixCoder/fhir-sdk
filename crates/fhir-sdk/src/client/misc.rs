//! Miscellaneous helpers.

use reqwest::header::{self, HeaderMap};

use super::Error;

/// Parse an ETag to a version ID.
pub fn parse_etag(headers: &HeaderMap) -> Result<String, Error> {
	let etag = headers
		.get(header::ETAG)
		.ok_or_else(|| Error::EtagFailure("None".to_owned()))?
		.to_str()
		.map_err(|err| Error::EtagFailure(err.to_string()))?;
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
		.to_str()
		.map_err(|err| Error::LocationFailure(err.to_string()))?;
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

#[cfg(test)]
mod tests {
	#![allow(clippy::expect_used)] // Allowed for tests

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
}

//! Comparison of JSON objects to be similar under the FHIR perspective.

use std::collections::HashSet;

use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde_json::{Map, Value};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Assert two JSON values to be similar under the FHIR perspective.
pub fn assert_fhir_json_equal(a: &Value, b: &Value) {
	if !fhir_json_equal(a, b) {
		let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);
		assert_json_matches!(a, b, config);
		panic!("FHIR JSON not equal, but assert_json_matches didn't catch it!");
	}
}

/// Compare two JSON values to be similar under the FHIR perspective.
fn fhir_json_equal(a: &Value, b: &Value) -> bool {
	match (a, b) {
		(Value::Null, Value::Null) => true,
		(Value::Bool(val_a), Value::Bool(val_b)) => val_a == val_b,
		(Value::Number(val_a), Value::Number(val_b)) => val_a.as_f64() == val_b.as_f64(),
		(Value::String(val_a), Value::String(val_b)) => strings_equal(val_a, val_b),
		(Value::Array(val_a), Value::Array(val_b)) => {
			val_a.iter().zip(val_b).all(|(a, b)| fhir_json_equal(a, b))
		}
		(Value::Object(val_a), Value::Object(val_b)) => map_equal(val_a, val_b),
		_ => false,
	}
}

/// Compare JSON strings to be equal under FHIR.
fn strings_equal(a: &str, b: &str) -> bool {
	let time_a = OffsetDateTime::parse(a, &Rfc3339).ok();
	let time_b = OffsetDateTime::parse(b, &Rfc3339).ok();
	match (time_a, time_b) {
		(Some(time_a), Some(time_b)) => return time_a == time_b,
		(None, None) => (),
		_ => return false,
	}

	let bytes_a = BASE64_STANDARD.decode(a.split_whitespace().collect::<String>()).ok();
	let bytes_b = BASE64_STANDARD.decode(b.split_whitespace().collect::<String>()).ok();
	match (bytes_a, bytes_b) {
		(Some(bytes_a), Some(bytes_b)) => return bytes_a == bytes_b,
		(None, None) => (),
		_ => return false,
	}

	a == b
}

/// Compare JSON maps to be equal.
fn map_equal(a: &Map<String, Value>, b: &Map<String, Value>) -> bool {
	let all_keys: HashSet<_> = a.keys().chain(b.keys()).collect();
	all_keys.into_iter().all(|key| match (a.get(key), b.get(key)) {
		(Some(val_a), Some(val_b)) => fhir_json_equal(val_a, val_b),
		(None, None) => true,
		_ => false,
	})
}

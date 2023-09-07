//! # FHIR Models.
//!
//! This is a sub-crate of [`fhir-sdk`](https://crates.io/crates/fhir-sdk). Please take a look at the main crate for
//! more documentation.

mod date_time;
mod error;
#[cfg(feature = "r4b")]
pub mod r4b;
#[cfg(feature = "r5")]
pub mod r5;

use std::ops::{Deref, DerefMut};

use base64::prelude::{Engine, BASE64_STANDARD_NO_PAD};
use serde::{Deserialize, Serialize};
pub use time;

pub use self::date_time::*;

/// Parsed parts of a FHIR reference. Can be one of local reference, relative
/// reference or absolute reference. The absolute reference is unchecked and can
/// be anything, it is used as fallback.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParsedReference<'a> {
	/// Local reference, the resource is to be found in the `contained` field.
	Local {
		/// Local ID of the resource.
		id: &'a str,
	},
	/// Relative reference, the resource is on the same FHIR server.
	Relative {
		/// Resource type.
		resource_type: &'a str,
		/// Resource ID.
		id: &'a str,
		/// Targeted version ID if set.
		version_id: Option<&'a str>,
	},
	/// Absolute reference, the resource can be anywhere.
	Absolute {
		/// Raw URL string.
		url: &'a str,
	},
}

/// FHIR `integer64` type. Wraps an i64, but serializes and deserializes as
/// string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Integer64(pub i64);

impl Serialize for Integer64 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.to_string().serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Integer64 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let i = s.parse().map_err(serde::de::Error::custom)?;
		Ok(Self(i))
	}
}

/// FHIR `base64Binary` type. Wraps binary data and serializes to base64
/// strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Base64Binary(pub Vec<u8>);

impl Serialize for Base64Binary {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let s = BASE64_STANDARD_NO_PAD.encode(&self.0);
		s.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Base64Binary {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let bytes = BASE64_STANDARD_NO_PAD.decode(s).map_err(serde::de::Error::custom)?;
		Ok(Self(bytes))
	}
}

/// Deref and From implementations for wrappers.
macro_rules! wrapper_impls {
	($wrapper:ident, $inner_type:ty) => {
		impl Deref for $wrapper {
			type Target = $inner_type;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl DerefMut for $wrapper {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		impl From<$inner_type> for $wrapper {
			fn from(inner: $inner_type) -> Self {
				Self(inner)
			}
		}

		impl From<$wrapper> for $inner_type {
			fn from(wrapper: $wrapper) -> $inner_type {
				wrapper.0
			}
		}
	};
}

wrapper_impls!(Integer64, i64);
wrapper_impls!(Base64Binary, Vec<u8>);
wrapper_impls!(Time, time::Time);
wrapper_impls!(Instant, time::OffsetDateTime);

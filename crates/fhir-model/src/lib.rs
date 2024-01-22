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
#[cfg(feature = "stu3")]
pub mod stu3;

use std::ops::{Deref, DerefMut};

use base64::prelude::{Engine, BASE64_STANDARD};
use serde::{Deserialize, Serialize};
pub use time;

pub use self::date_time::*;

/// Parsed parts of a FHIR reference. Can be one of local reference, relative
/// reference or absolute reference. The absolute reference is unchecked and can
/// be anything, it is used as fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
	/// Absolute reference, the resource can be anywhere, might not even be FHIR
	/// server. Might not be a URL, but a URI like a `urn:uuid:...`.
	Absolute {
		/// Raw URL string.
		url: &'a str,
		/// Assumed resource type part if exists. Is just the positional URL
		/// segment, could be wrong.
		resource_type: Option<&'a str>,
		/// Assumed resource ID part if exists. Is just the positional URL
		/// segment, could be wrong.
		id: Option<&'a str>,
	},
}

impl<'a> ParsedReference<'a> {
	/// Parse the reference from a reference string.
	#[must_use]
	pub fn new(reference: &'a str) -> Self {
		if reference.starts_with('#') {
			return Self::Local { id: reference.split_at(1).1 };
		}

		let Some((resource_type, id, version_id, is_absolute)) = Self::parse_segments(reference)
		else {
			return Self::Absolute { url: reference, resource_type: None, id: None };
		};

		if is_absolute {
			Self::Absolute { url: reference, resource_type: Some(resource_type), id: Some(id) }
		} else {
			Self::Relative { resource_type, id, version_id }
		}
	}

	/// Helper function to split the reference segments if possible.
	/// Returns resource type, id, version id and if there is more segments if
	/// possible.
	fn parse_segments(reference: &'a str) -> Option<(&'a str, &'a str, Option<&'a str>, bool)> {
		let mut segments = reference.rsplit('/');
		let id_or_version = segments.next()?;
		let history_or_type = segments.next()?;
		Some(if history_or_type == "_history" {
			let id = segments.next()?;
			let resource_type = segments.next()?;
			(resource_type, id, Some(id_or_version), segments.next().is_some())
		} else {
			(history_or_type, id_or_version, None, segments.next().is_some())
		})
	}

	/// Get the resource type that this reference points to as string reference.
	/// Might not be present/visible in absolute URLs or local references. In
	/// absolute URLs, it might also return garbage, as the URL might have
	/// enough segments, but the segment was not an actual resource type. Take
	/// care when parsing to `ResourceType`.
	#[must_use]
	pub fn resource_type(&self) -> Option<&'a str> {
		match self {
			Self::Local { .. } => None,
			Self::Relative { resource_type, .. } => Some(resource_type),
			Self::Absolute { resource_type, .. } => *resource_type,
		}
	}

	/// Get the resource ID that this reference points to. Might not be
	/// present/visible in absolute URLs. In absolute URLs, it might also return
	/// garbage, as the URL might have enough segments, but the segment was not
	/// an actual id. Take care when using it.
	#[must_use]
	pub fn id(&self) -> Option<&'a str> {
		match self {
			Self::Local { id } => Some(id),
			Self::Relative { id, .. } => Some(id),
			Self::Absolute { id, .. } => *id,
		}
	}
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
		let s = BASE64_STANDARD.encode(&self.0);
		s.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Base64Binary {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let mut s = String::deserialize(deserializer)?;
		s.retain(|c| !c.is_whitespace());
		let bytes = BASE64_STANDARD.decode(s).map_err(serde::de::Error::custom)?;
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

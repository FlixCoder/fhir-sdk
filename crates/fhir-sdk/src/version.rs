//! Marker types for FHIR versions. Mainly for use in the client (but also
//! extension traits).

use std::{
	fmt::{Debug, Display},
	str::FromStr,
};

use fhir_model::for_all_versions;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
	extensions::{BundleEntryExt, BundleExt, GenericResource, ReferenceExt},
	utils::Sealed,
};

/// FHIR version STU3, "3.0".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FhirStu3;
/// FHIR version R4B, "4.3".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FhirR4B;
/// FHIR version R5, "5.0".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FhirR5;

#[cfg(feature = "r5")]
/// Default FHIR version (e.g. in client).
pub type DefaultVersion = FhirR5;
#[cfg(all(not(feature = "r5"), feature = "r4b"))]
/// Default FHIR version (e.g. in client).
pub type DefaultVersion = FhirR4B;
#[cfg(all(not(feature = "r5"), not(feature = "r4b"), feature = "stu3"))]
/// Default FHIR version (e.g. in client).
pub type DefaultVersion = FhirStu3;

/// Internal macro to convert the module version identifier to the FHIR version
/// type.
macro_rules! fhir_version {
	(stu3) => {
		$crate::version::FhirStu3
	};
	(r4b) => {
		$crate::version::FhirR4B
	};
	(r5) => {
		$crate::version::FhirR5
	};
}
pub(crate) use fhir_version;

/// FHIR version type "marker", but with additional information.
pub trait FhirVersion: Sealed + Unpin + Send + Sync + 'static {
	/// FHIR version string.
	const VERSION: &'static str;
	/// JSON mime type used by this version.
	const MIME_TYPE: &'static str;

	/// `ResourceType` of this version.
	type ResourceType: Serialize
		+ DeserializeOwned
		+ Debug
		+ FromStr
		+ AsRef<str>
		+ Display
		+ Clone
		+ Copy
		+ PartialEq
		+ Eq
		+ Unpin
		+ Send
		+ Sync;
	/// Generic `Resource` enum of this version.
	type Resource: GenericResource
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;

	/// `Bundle` resource.
	type Bundle: BundleExt<Entry: BundleEntryExt<Resource = Self::Resource>>
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;
	/// `CapabilityStatement` resource.
	type CapabilityStatement: Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;
	/// `OperationOutcome` resource.
	type OperationOutcome: Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;

	/// `Reference` type.
	type Reference: ReferenceExt
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;

	/// `SearchComparator` type.
	type SearchComparator: Serialize
		+ DeserializeOwned
		+ Debug
		+ FromStr
		+ AsRef<str>
		+ Display
		+ Clone
		+ Copy
		+ PartialEq
		+ Eq
		+ Unpin
		+ Send
		+ Sync;
}

impl Sealed for FhirStu3 {}
impl Sealed for FhirR4B {}
impl Sealed for FhirR5 {}

/// Implement `FhirVersion` for all supported versions.
macro_rules! impl_fhir_version {
	($version:ident) => {
		use fhir_model::$version;

		impl FhirVersion for fhir_version!($version) {
			const VERSION: &'static str = $version::VERSION;
			const MIME_TYPE: &'static str = $version::JSON_MIME_TYPE;

			type ResourceType = $version::resources::ResourceType;
			type Resource = $version::resources::Resource;

			type Bundle = $version::resources::Bundle;
			type CapabilityStatement = $version::resources::CapabilityStatement;
			type OperationOutcome = $version::resources::OperationOutcome;

			type Reference = $version::types::Reference;

			type SearchComparator = $version::codes::SearchComparator;
		}
	};
}
for_all_versions!(impl_fhir_version);

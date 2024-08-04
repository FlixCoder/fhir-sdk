//! Marker types for FHIR versions. Mainly for use in the client (but also
//! extension traits).

use crate::utils::Sealed;

/// FHIR version STU3, "3.0".
#[derive(Debug)]
pub struct FhirStu3;
/// FHIR version R4B, "4.3".
#[derive(Debug)]
pub struct FhirR4B;
/// FHIR version R5, "5.0".
#[derive(Debug)]
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

/// FHIR version type "marker", but with additional information.
pub trait FhirVersion: Sealed + Send + Sync {
	/// FHIR version string.
	const VERSION: &'static str;
	/// JSON mime type used by this version.
	const MIME_TYPE: &'static str;
}

impl Sealed for FhirStu3 {}
impl FhirVersion for FhirStu3 {
	const VERSION: &'static str = fhir_model::stu3::VERSION;
	const MIME_TYPE: &'static str = fhir_model::stu3::JSON_MIME_TYPE;
}

impl Sealed for FhirR4B {}
impl FhirVersion for FhirR4B {
	const VERSION: &'static str = fhir_model::r4b::VERSION;
	const MIME_TYPE: &'static str = fhir_model::r4b::JSON_MIME_TYPE;
}

impl Sealed for FhirR5 {}
impl FhirVersion for FhirR5 {
	const VERSION: &'static str = fhir_model::r5::VERSION;
	const MIME_TYPE: &'static str = fhir_model::r5::JSON_MIME_TYPE;
}

/// Internal macro to convert the module version identifier to the FHIR version
/// type.
macro_rules! fhir_version {
	(stu3) => {
		FhirStu3
	};
	(r4b) => {
		FhirR4B
	};
	(r5) => {
		FhirR5
	};
}
pub(crate) use fhir_version;

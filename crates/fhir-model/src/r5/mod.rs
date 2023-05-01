//! Revision 5 types of FHIR.

pub mod codes;
pub mod resources;
pub mod types;

/// Numeric version string of this FHIR version (e.g. or mime-type).
pub const VERSION: &str = "5.0";
/// FHIR MIME-type this version uses for JSON.
pub const JSON_MIME_TYPE: &str = "application/fhir+json; fhirVersion=5.0";

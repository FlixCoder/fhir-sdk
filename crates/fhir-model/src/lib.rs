//! # FHIR Models.
//!
//! This is a sub-crate of `fhir-sdk`. Please take a look at the main crate for
//! more documentation.

#[cfg(feature = "r4b")]
pub mod r4b;
#[cfg(feature = "r5")]
pub mod r5;

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

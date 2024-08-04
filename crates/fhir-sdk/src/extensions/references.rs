//! Generalized functionality for references.

use fhir_model::{for_all_versions, ParsedReference};

/// Additional functionality for `Reference`s.
pub trait ReferenceExt {
	/// Parse into [`ParsedReference`].
	fn parse(&self) -> Option<ParsedReference<'_>>;
	/// Get the defined type, if there is one.
	fn r#type(&self) -> Option<&str>;
}

macro_rules! impl_reference_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::types::Reference;

			use super::*;

			impl ReferenceExt for Reference {
				fn parse(&self) -> Option<ParsedReference<'_>> {
					Self::parse(self)
				}

				fn r#type(&self) -> Option<&str> {
					impl_reference_ext!(@get_type $version self)
				}
			}
		}
	};
	(@get_type stu3 $s:expr) => {
		None
	};
	(@get_type $version:ident $s:expr) => {
		$s.r#type.as_deref()
	};
}
for_all_versions!(impl_reference_ext);

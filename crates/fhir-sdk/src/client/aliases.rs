//! Version aliases for easier use of version specific types. Also for
//! compatibility with the previous implementation.

use fhir_model::for_all_versions;

/// Implement type aliases for search parameters for all versions.
macro_rules! impl_search_param_aliases {
	($version:ident) => {
		pub mod $version {
			//! Aliases for types using the specified FHIR version.

			pub use super::super::{MissingSearch, StringSearch, TokenSearch, UriSearch};
			use crate::version::fhir_version;

			/// Alias for `NumberSearch` for this FHIR version.
			pub type NumberSearch<'a> =
				super::super::fhir::NumberSearch<'a, fhir_version!($version)>;
			/// Alias for `DateSearch` for this FHIR version.
			pub type DateSearch<'a> = super::super::fhir::DateSearch<'a, fhir_version!($version)>;
			/// Alias for `ReferenceSearch` for this FHIR version.
			pub type ReferenceSearch<'a> =
				super::super::fhir::ReferenceSearch<'a, fhir_version!($version)>;
			/// Alias for `QuantitySearch` for this FHIR version.
			pub type QuantitySearch<'a> =
				super::super::fhir::QuantitySearch<'a, fhir_version!($version)>;
		}
	};
}
for_all_versions!(impl_search_param_aliases);

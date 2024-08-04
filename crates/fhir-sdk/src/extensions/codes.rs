//! General functionality on codes.

use fhir_model::for_all_versions;

/// `SearchEntryMode` functionality.
pub trait SearchEntryModeExt {
	/// Check if the search mode is `Match`.
	fn is_match(&self) -> bool;
}

/// Implement `SearchEntryModeExt` for all versions.
macro_rules! impl_search_entry_mode_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::codes::SearchEntryMode;

			use super::*;

			impl SearchEntryModeExt for SearchEntryMode {
				fn is_match(&self) -> bool {
					*self == SearchEntryMode::Match
				}
			}
		}
	};
}
for_all_versions!(impl_search_entry_mode_ext);

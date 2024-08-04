//! Implementations on `Bundle`s.

use crate::for_all_versions;

/// Link relation for the next page.
macro_rules! bundle_next_link_relation {
	(stu3) => {
		"next"
	};
	(r4b) => {
		"next"
	};
	(r5) => {
		$crate::r5::codes::LinkRelationTypes::Next
	};
}

/// Implement `Bundle` functions for all FHIR versions.
macro_rules! impl_bundle {
	($version:ident) => {
		mod $version {
			use $crate::$version::resources::Bundle;

			impl Bundle {
				/// Assuming the bundle is a search result, get the next page URL if
				/// there is one.
				#[must_use]
				pub fn next_page_url(&self) -> Option<&String> {
					self.link
						.iter()
						.flatten()
						.find(|link| link.relation == bundle_next_link_relation!($version))
						.map(|link| &link.url)
				}
			}
		}
	};
}
for_all_versions!(impl_bundle);

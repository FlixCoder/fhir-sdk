//! Bundle extension.

/// Extended functionality on `Bundle`s.
pub trait BundleExt {
	/// Assuming the bundle is a search result, get the next page URL if there
	/// is one.
	fn next_page_url(&self) -> Option<&String>;
}

/// Implements the trait for all FHIR versions.
macro_rules! implement_bundle_ext {
	(r5) => {
		implement_bundle_ext!(@r5, $crate::r5::codes::LinkRelationTypes::Next);
	};
	(r4b) => {
		implement_bundle_ext!(@r4b, "next");
	};
	(stu3) => {
		implement_bundle_ext!(@stu3, "next");
	};

	(@ $version:ident, $link_relation:expr) => {
		mod $version {
			use $crate::$version::resources::Bundle;

			use super::*;

			impl BundleExt for Bundle {
				fn next_page_url(&self) -> Option<&String> {
					self.link
						.iter()
						.flatten()
						.find(|link| link.relation == $link_relation)
						.map(|link| &link.url)
				}
			}
		}
	};
}

for_all_versions!(implement_bundle_ext);

//! Implementations on `ResourceType`s.

use crate::for_all_versions;

/// Implement `ResourceType` functions/traits for all FHIR versions.
macro_rules! impl_resource_type {
	($version:ident) => {
		mod $version {
			use $crate::$version::resources::ResourceType;

			impl AsRef<str> for ResourceType {
				fn as_ref(&self) -> &str {
					self.as_str()
				}
			}
		}
	};
}
for_all_versions!(impl_resource_type);

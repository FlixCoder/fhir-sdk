//! Extensions on generic resource enum.

use fhir_model::for_all_versions;

/// Extended/shared functionality for generic resource enums.
#[allow(dead_code)] // For future usage.
pub trait GenericResource {
	/// Get the resource type as str.
	fn resource_type_str(&self) -> &str;
	/// Get the ID of the resource.
	fn id(&self) -> Option<&str>;
	/// Set the version of the resource.
	fn set_id(&mut self, id: String);
	/// Get the version ID of the resource.
	fn version_id(&self) -> Option<&str>;
	/// Set the version ID of the resource.
	fn set_version_id(&mut self, version_id: String);
}

/// Implement the generic resource trait for all versions.
macro_rules! impl_generic_resource {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::{resources::Resource, types::Meta};

			use super::*;

			impl GenericResource for Resource {
				#[inline]
				fn resource_type_str(&self) -> &str {
					self.resource_type().as_str()
				}

				#[inline]
				fn id(&self) -> Option<&str> {
					self.as_base_resource().id().as_deref()
				}

				#[inline]
				fn set_id(&mut self, id: String) {
					self.as_base_resource_mut().set_id(Some(id));
				}

				#[inline]
				fn version_id(&self) -> Option<&str> {
					self.as_base_resource()
						.meta()
						.as_ref()
						.and_then(|meta| meta.version_id.as_deref())
				}

				#[inline]
				fn set_version_id(&mut self, version_id: String) {
					if let Some(meta) = self.as_base_resource_mut().meta_mut() {
						meta.version_id = Some(version_id);
					} else {
						// Meta does not require any field and will succeed building.
						#[allow(clippy::unwrap_used)]
						self.as_base_resource_mut().set_meta(Some(
							Meta::builder().version_id(version_id).build().unwrap(),
						));
					}
				}
			}
		}
	};
}
for_all_versions!(impl_generic_resource);

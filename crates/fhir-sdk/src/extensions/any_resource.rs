//! Traits for all resources.

use fhir_model::for_all_versions;

use crate::version::{fhir_version, FhirVersion};

/// Basic trait to combine all resources from all FHIR versions to one.
/// Especially for use in the client to accept and handly any resource.
/// Only implemented if "builders" feature is active.
pub trait AnyResource<V: FhirVersion> {
	/// ResourceType of this resource.
	const TYPE: V::ResourceType;
	/// Resource type of the resource as `&str`. Must be valid for use in URLs.
	const TYPE_STR: &'static str;

	/// Get the resource's ID as string.
	fn id(&self) -> Option<&str>;
	/// Set the resource's ID.
	fn set_id(&mut self, id: String);
	/// Get the resource's version ID as string.
	fn version_id(&self) -> Option<&str>;
	/// Set the resource's version ID.
	fn set_version_id(&mut self, version_id: String);
}

/// Implement `AnyResource` for all fhir versions.
macro_rules! impl_any_resource {
	($version:ident) => {
		use fhir_model::$version;

		impl<R> AnyResource<fhir_version!($version)> for R
		where
			R: $version::resources::NamedResource + $version::resources::BaseResource,
		{
			const TYPE: $version::resources::ResourceType = R::TYPE;
			const TYPE_STR: &'static str = R::TYPE.as_str();

			#[inline]
			fn id(&self) -> Option<&str> {
				$version::resources::BaseResource::id(self).as_deref()
			}

			#[inline]
			fn set_id(&mut self, id: String) {
				$version::resources::BaseResource::set_id(self, Some(id));
			}

			#[inline]
			fn version_id(&self) -> Option<&str> {
				self.meta().as_ref().and_then(|meta| meta.version_id.as_deref())
			}

			#[inline]
			fn set_version_id(&mut self, version_id: String) {
				if let Some(meta) = self.meta_mut() {
					meta.version_id = Some(version_id);
				} else {
					// Meta does not require any field and will succeed building.
					#[allow(clippy::unwrap_used)]
					self.set_meta(Some(
						$version::types::Meta::builder().version_id(version_id).build().unwrap(),
					));
				}
			}
		}
	};
}
#[cfg(feature = "builders")]
for_all_versions!(impl_any_resource);

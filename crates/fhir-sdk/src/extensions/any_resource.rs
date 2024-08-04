//! Traits for all resources.

use fhir_model::for_all_versions;

use crate::version::{fhir_version, FhirR4B, FhirR5, FhirStu3};

/// Basic trait to combine all resources from all FHIR versions to one.
/// Especially for use in the client to accept and handly any resource.
pub trait AnyResource<Version> {
	/// The resource type of the resource as `&str`. Must be valid for use in
	/// URLs.
	const TYPE_STR: &'static str;

	/// Get the resource's ID as string.
	fn id(&self) -> Option<&str>;
	/// Get the resource's version ID as string.
	fn version_id(&self) -> Option<&str>;
}

/// Implement `AnyResource` for all fhir versions.
macro_rules! impl_any_resource {
	($version:ident) => {
		use fhir_model::$version::resources as $version;

		impl<R> AnyResource<fhir_version!($version)> for R
		where
			R: $version::NamedResource + $version::BaseResource,
		{
			const TYPE_STR: &'static str = R::TYPE.as_str();

			fn id(&self) -> Option<&str> {
				$version::BaseResource::id(self).as_deref()
			}

			fn version_id(&self) -> Option<&str> {
				$version::BaseResource::meta(self)
					.as_ref()
					.and_then(|meta| meta.version_id.as_deref())
			}
		}
	};
}

for_all_versions!(impl_any_resource);

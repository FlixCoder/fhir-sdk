//! Revision 4B types of FHIR.

pub mod codes;
pub mod resources;
pub mod types;

use self::{
	resources::{BaseResource, NamedResource},
	types::{Reference, ReferenceInner},
};

/// Numeric version string of this FHIR version (e.g. or mime-type).
pub const VERSION: &str = "4.3";
/// FHIR MIME-type this version uses for JSON.
pub const JSON_MIME_TYPE: &str = "application/fhir+json; fhirVersion=4.3";

/// Create relative [`Reference`] to the given resource.
pub fn reference_to<R>(resource: &R) -> Option<Reference>
where
	R: NamedResource + BaseResource,
{
	Some(
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: Some(format!("{}/{}", R::TYPE, resource.id().as_ref()?)),
			reference_ext: None,
			r#type: Some(R::TYPE.to_string()),
			type_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
		.into(),
	)
}

/// Create local [`Reference`] to the given resource. Make sure the resource is
/// going to be in the `contained` field of the referencing resource.
pub fn local_reference_to<R>(resource: &R) -> Option<Reference>
where
	R: NamedResource + BaseResource,
{
	Some(
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: Some(format!("#{}", resource.id().as_ref()?)),
			reference_ext: None,
			r#type: Some(R::TYPE.to_string()),
			type_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
		.into(),
	)
}

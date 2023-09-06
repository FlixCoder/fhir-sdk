//! Extended [`Reference`] functionality.

use super::{
	resources::{BaseResource, NamedResource},
	types::{Reference, ReferenceInner},
};

/// Create relative [`Reference`] to the given resource.
pub fn create_to<R>(resource: R) -> Option<Reference>
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
pub fn create_local_to<R>(resource: R) -> Option<Reference>
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

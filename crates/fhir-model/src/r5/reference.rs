//! Extended [`Reference`] functionality.

use super::{
	resources::{BaseResource, NamedResource},
	types::{Reference, ReferenceInner},
};
use crate::ParsedReference;

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

impl Reference {
	/// Parse the [`Reference`] into a [`ParsedReference`]. Returns `None` if
	/// the `reference` field is empty or the URL does not contain enough
	/// segments to be valid.
	#[must_use]
	pub fn parse(&self) -> Option<ParsedReference<'_>> {
		let url = self.reference.as_ref()?;

		if url.starts_with('#') {
			return Some(ParsedReference::Local { id: url.split_at(1).1 });
		}

		let mut segments = url.rsplit('/');
		let id_or_version = segments.next()?;
		let history_or_type = segments.next()?;
		let (resource_type, id, version_id) = if history_or_type == "_history" {
			let id = segments.next()?;
			let resource_type = segments.next()?;
			(resource_type, id, Some(id_or_version))
		} else {
			(history_or_type, id_or_version, None)
		};

		if segments.next().is_some() {
			Some(ParsedReference::Absolute { url })
		} else {
			Some(ParsedReference::Relative { resource_type, id, version_id })
		}
	}
}

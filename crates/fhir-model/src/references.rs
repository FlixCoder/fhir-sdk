//! References implementations.

use crate::for_all_versions;

/// Parsed parts of a FHIR reference. Can be one of local reference, relative
/// reference or absolute reference. The absolute reference is unchecked and can
/// be anything, it is used as fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedReference<'a> {
	/// Local reference, the resource is to be found in the `contained` field.
	Local {
		/// Local ID of the resource.
		id: &'a str,
	},
	/// Relative reference, the resource is on the same FHIR server.
	Relative {
		/// Resource type.
		resource_type: &'a str,
		/// Resource ID.
		id: &'a str,
		/// Targeted version ID if set.
		version_id: Option<&'a str>,
	},
	/// Absolute reference, the resource can be anywhere, might not even be FHIR
	/// server. Might not be a URL, but a URI like a `urn:uuid:...`.
	Absolute {
		/// Raw URL string.
		url: &'a str,
		/// Assumed resource type part if exists. Is just the positional URL
		/// segment, could be wrong.
		resource_type: Option<&'a str>,
		/// Assumed resource ID part if exists. Is just the positional URL
		/// segment, could be wrong.
		id: Option<&'a str>,
	},
}

impl<'a> ParsedReference<'a> {
	/// Parse the reference from a reference string.
	#[must_use]
	pub fn new(reference: &'a str) -> Self {
		if reference.starts_with('#') {
			return Self::Local { id: reference.split_at(1).1 };
		}

		let Some((resource_type, id, version_id, is_absolute)) = Self::parse_segments(reference)
		else {
			return Self::Absolute { url: reference, resource_type: None, id: None };
		};

		if is_absolute {
			Self::Absolute { url: reference, resource_type: Some(resource_type), id: Some(id) }
		} else {
			Self::Relative { resource_type, id, version_id }
		}
	}

	/// Helper function to split the reference segments if possible.
	/// Returns resource type, id, version id and if there is more segments if
	/// possible.
	fn parse_segments(reference: &'a str) -> Option<(&'a str, &'a str, Option<&'a str>, bool)> {
		let mut segments = reference.rsplit('/');
		let id_or_version = segments.next()?;
		let history_or_type = segments.next()?;
		Some(if history_or_type == "_history" {
			let id = segments.next()?;
			let resource_type = segments.next()?;
			(resource_type, id, Some(id_or_version), segments.next().is_some())
		} else {
			(history_or_type, id_or_version, None, segments.next().is_some())
		})
	}

	/// Get the resource type that this reference points to as string reference.
	/// Might not be present/visible in absolute URLs or local references. In
	/// absolute URLs, it might also return garbage, as the URL might have
	/// enough segments, but the segment was not an actual resource type. Take
	/// care when parsing to `ResourceType`.
	#[must_use]
	pub const fn resource_type(&self) -> Option<&'a str> {
		match self {
			Self::Local { .. } => None,
			Self::Relative { resource_type, .. } => Some(resource_type),
			Self::Absolute { resource_type, .. } => *resource_type,
		}
	}

	/// Get the resource ID that this reference points to. Might not be
	/// present/visible in absolute URLs. In absolute URLs, it might also return
	/// garbage, as the URL might have enough segments, but the segment was not
	/// an actual id. Take care when using it.
	#[must_use]
	pub const fn id(&self) -> Option<&'a str> {
		match self {
			Self::Local { id } => Some(id),
			Self::Relative { id, .. } => Some(id),
			Self::Absolute { id, .. } => *id,
		}
	}
}

/// Construct a `ReferenceInner` for the given version.
macro_rules! make_reference_inner {
	(stu3, $reference:expr, $type:expr) => {
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: $reference,
			reference_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
	};
	(r4b, $reference:expr, $type:expr) => {
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: $reference,
			reference_ext: None,
			r#type: $type,
			type_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
	};
	(r5, $reference:expr, $type:expr) => {
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: $reference,
			reference_ext: None,
			r#type: $type,
			type_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
	};
}

/// Macro to implement functions on `Reference`s.
macro_rules! impl_reference {
	($version:ident) => {
		mod $version {
			use $crate::$version::{
				resources::{BaseResource, NamedResource, ResourceType},
				types::{Reference, ReferenceInner},
			};

			use super::ParsedReference;

			impl Reference {
				/// Parse the [`Reference`] into a [`ParsedReference`]. Returns `None`
				/// if the `reference` field is empty..
				#[must_use]
				pub fn parse(&self) -> Option<ParsedReference<'_>> {
					let url = self.reference.as_ref()?;
					Some(ParsedReference::new(url))
				}

				/// Create a new local reference for the given ID. Make sure the
				/// resource is going to be in the `contained` field of the
				/// referencing resource.
				#[must_use]
				#[allow(unused_variables, reason = "Unused for STU3")]
				pub fn local(ty: ResourceType, id: &str) -> Self {
					make_reference_inner!($version, Some(format!("#{id}")), Some(ty.to_string()))
						.into()
				}

				/// Create local [`Reference`] to the given resource. Make sure the
				/// resource is going to be in the `contained` field of the
				/// referencing resource.
				pub fn local_to<R>(resource: &R) -> Option<Self>
				where
					R: NamedResource + BaseResource,
				{
					Some(Self::local(R::TYPE, resource.id().as_ref()?))
				}

				/// Create a new relative reference for the given resource type and ID.
				#[must_use]
				pub fn relative(ty: ResourceType, id: &str) -> Self {
					make_reference_inner!(
						$version,
						Some(format!("{ty}/{id}")),
						Some(ty.to_string())
					)
					.into()
				}

				/// Create relative [`Reference`] to the given resource.
				pub fn relative_to<R>(resource: &R) -> Option<Self>
				where
					R: NamedResource + BaseResource,
				{
					Some(Self::relative(R::TYPE, resource.id().as_ref()?))
				}
			}
		}
	};
}
for_all_versions!(impl_reference);

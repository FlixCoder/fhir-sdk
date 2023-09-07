//! Type definitions.

#[rustfmt::skip] // Too much for rustfmt
mod generated;

pub use generated::*;

use crate::ParsedReference;

impl CodeableConcept {
	/// Get all codes matching a specific system.
	pub fn codes_with_system<'a, 'b>(
		&'a self,
		system: &'b str,
	) -> impl Iterator<Item = &'a str> + Send + 'b
	where
		'a: 'b,
	{
		self.coding
			.iter()
			.flatten()
			.filter(|coding| coding.system.as_deref() == Some(system))
			.filter_map(|coding| coding.code.as_deref())
	}

	/// Get the first code matching a specific system.
	#[must_use]
	pub fn code_with_system<'a>(&'a self, system: &str) -> Option<&'a str> {
		self.codes_with_system(system).next()
	}
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

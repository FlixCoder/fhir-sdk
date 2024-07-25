//! Type definitions.

#[rustfmt::skip] // Too much for rustfmt
#[allow(clippy::doc_lazy_continuation)] // Comments from FHIR spec.
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
	/// the `reference` field is empty.
	#[must_use]
	pub fn parse(&self) -> Option<ParsedReference<'_>> {
		let url = self.reference.as_ref()?;
		Some(ParsedReference::new(url))
	}
}

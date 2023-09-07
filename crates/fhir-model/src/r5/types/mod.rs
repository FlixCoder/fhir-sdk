//! Type definitions.

#[rustfmt::skip] // Too much for rustfmt
mod generated;

pub use generated::*;

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

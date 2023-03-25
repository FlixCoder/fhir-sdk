//! ValueSets (CodeSystems) parsing for FHIR codes.

use fhirbolt::model::r4b::{
	resources::{Bundle, CodeSystem, CodeSystemConcept},
	Resource,
};

use crate::utils::Status;

/// Code definition.
#[derive(Debug)]
pub struct Code {
	/// Name of the code.
	pub name: String,
	/// FHIR version.
	pub version: Option<String>,
	/// Description.
	pub description: Option<String>,
	/// Status of the definition.
	pub status: Status,
	/// Whether it is experimental.
	pub experimental: bool,
	/// Whether the codes are case sensitive.
	pub case_sensitive: bool,
	/// Whether this code is a value set (controls whether there are custom
	/// values allowed I think?).
	pub is_value_set: bool,
	/// Code system.
	pub system: String,
	/// Code items:
	pub items: Vec<CodeItem>,
}

impl From<CodeSystem> for Code {
	fn from(code_system: CodeSystem) -> Self {
		let name = code_system.name.and_then(|v| v.value).expect("CodeSystem.name");
		let version = code_system.version.and_then(|v| v.value);
		let description = code_system.description.and_then(|v| v.value);
		let status = code_system
			.status
			.value
			.expect("CodeSystem.status")
			.parse()
			.expect("parsing CodeSystem.status");
		let experimental =
			code_system.experimental.and_then(|v| v.value).expect("CodeSystem.experimental");
		let case_sensitive = code_system.case_sensitive.and_then(|v| v.value).unwrap_or(false);
		let is_value_set = code_system.value_set.is_some();
		let system = code_system
			.value_set
			.and_then(|v| v.value)
			.or(code_system.url.and_then(|v| v.value))
			.expect("CodeSystem.valueSet or CodeSystem.url");

		let items = code_system
			.concept
			.into_iter()
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.map(CodeItem::from)
			.collect();

		Self {
			name,
			version,
			description,
			status,
			experimental,
			case_sensitive,
			is_value_set,
			system,
			items,
		}
	}
}

/// Code item information.
#[derive(Debug)]
pub struct CodeItem {
	/// The value.
	pub code: String,
	/// The human version to be displayed.
	pub display: Option<String>,
	/// Definition of the value.
	pub definition: Option<String>,
}

impl From<CodeSystemConcept> for CodeItem {
	fn from(concept: CodeSystemConcept) -> Self {
		let code = concept.code.value.expect("CodeSystem.concept.code");
		let display = concept.display.and_then(|v| v.value);
		let definition = concept.definition.and_then(|v| v.value);

		Self { code, display, definition }
	}
}

/// Parse a Bundle into Codes.
pub fn parse(input: &str) -> Vec<Code> {
	let bundle: Bundle = fhirbolt::json::from_str(input, None).expect("Deserializing codes Bundle");

	bundle
		.entry
		.into_iter()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match *resource {
			Resource::CodeSystem(code_system) => Some(code_system),
			_ => None,
		})
		.map(|code_system| Code::from(*code_system))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_value_sets_from_code_systems() {
		let included = include_str!("../definitions/r4b/valuesets.json");
		let _codes = parse(included);
	}
}

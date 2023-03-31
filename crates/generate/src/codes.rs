//! ValueSets (CodeSystems) parsing for FHIR codes.

use fhir_model::r4b::{
	codes::PublicationStatus,
	resources::{Bundle, CodeSystem, CodeSystemConcept, Resource},
};

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
	pub status: PublicationStatus,
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
		let code_system = code_system.0;
		let name = code_system.name.expect("CodeSystem.name");
		let version = code_system.version;
		let description = code_system.description;
		let status = code_system.status;
		let experimental = code_system.experimental.expect("CodeSystem.experimental");
		let case_sensitive = code_system.case_sensitive.unwrap_or(false);
		let is_value_set = code_system.value_set.is_some();
		let system = code_system
			.value_set
			.or(code_system.url)
			.expect("CodeSystem.valueSet or CodeSystem.url");

		let items = code_system
			.concept
			.into_iter()
			.flatten()
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).flatten().collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).flatten().collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).flatten().collect();
				[concept].into_iter().chain(inner_concepts)
			})
			.flat_map(|mut concept| {
				let inner_concepts: Vec<_> = concept.concept.drain(..).flatten().collect();
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
		let code = concept.code;
		let display = concept.display;
		let definition = concept.definition;

		Self { code, display, definition }
	}
}

/// Parse a Bundle into Codes.
pub fn parse(input: &str) -> Vec<Code> {
	let bundle: Bundle = serde_json::from_str(input).expect("Deserializing codes Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			Resource::CodeSystem(code_system) => Some(code_system),
			_ => None,
		})
		.map(Code::from)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_value_sets_from_code_systems() {
		let included = include_str!("../definitions/r4b/valuesets.json");
		let _codes = parse(included);
		let included = include_str!("../definitions/r5/valuesets.json");
		let _codes = parse(included);
	}
}

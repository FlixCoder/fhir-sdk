//! Codes parsing from CodeSystems.

use fhir_model::{r4b, r5, stu3};

use crate::model::codes::{Code, CodeItem};

impl From<stu3::resources::CodeSystem> for Code {
	fn from(code_system: stu3::resources::CodeSystem) -> Self {
		let code_system = code_system.0;
		let name = code_system.name.expect("CodeSystem.name");
		let version = code_system.version;
		let description = code_system.description;
		let status = code_system.status.into();
		let experimental = code_system.experimental.expect("CodeSystem.experimental");
		let case_sensitive = code_system.case_sensitive.unwrap_or(false);
		let is_value_set = code_system.value_set.is_some();
		let system = code_system
			.value_set
			.or(code_system.url)
			.expect("CodeSystem.valueSet or CodeSystem.url");
		// Split off version string at the end (|5.0.0).
		let system =
			system.split_once('|').map_or(system.as_str(), |(start, _end)| start).to_owned();
		let content = code_system.content.into();

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
			content,
			items,
		}
	}
}

impl From<r4b::resources::CodeSystem> for Code {
	fn from(code_system: r4b::resources::CodeSystem) -> Self {
		let code_system = code_system.0;
		let name = code_system.name.expect("CodeSystem.name");
		let version = code_system.version;
		let description = code_system.description;
		let status = code_system.status.into();
		let experimental = code_system.experimental.expect("CodeSystem.experimental");
		let case_sensitive = code_system.case_sensitive.unwrap_or(false);
		let is_value_set = code_system.value_set.is_some();
		let system = code_system
			.value_set
			.or(code_system.url)
			.expect("CodeSystem.valueSet or CodeSystem.url");
		// Split off version string at the end (|5.0.0).
		let system =
			system.split_once('|').map_or(system.as_str(), |(start, _end)| start).to_owned();
		let content = code_system.content.into();

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
			content,
			items,
		}
	}
}

impl From<r5::resources::CodeSystem> for Code {
	fn from(code_system: r5::resources::CodeSystem) -> Self {
		let code_system = code_system.0;
		let name = code_system.name.expect("CodeSystem.name");
		let version = code_system.version;
		let description = code_system.description;
		let status = code_system.status.into();
		let experimental = code_system.experimental.expect("CodeSystem.experimental");
		let case_sensitive = code_system.case_sensitive.unwrap_or(false);
		let is_value_set = code_system.value_set.is_some();
		let system = code_system
			.value_set
			.or(code_system.url)
			.expect("CodeSystem.valueSet or CodeSystem.url");
		// Split off version string at the end (|5.0.0).
		let system =
			system.split_once('|').map_or(system.as_str(), |(start, _end)| start).to_owned();
		let content = code_system.content.into();

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
			content,
			items,
		}
	}
}

impl From<stu3::resources::CodeSystemConcept> for CodeItem {
	fn from(concept: stu3::resources::CodeSystemConcept) -> Self {
		let code = concept.code;
		let display = concept.display;
		let definition = concept.definition;

		Self { code, display, definition }
	}
}

impl From<r4b::resources::CodeSystemConcept> for CodeItem {
	fn from(concept: r4b::resources::CodeSystemConcept) -> Self {
		let code = concept.code;
		let display = concept.display;
		let definition = concept.definition;

		Self { code, display, definition }
	}
}

impl From<r5::resources::CodeSystemConcept> for CodeItem {
	fn from(concept: r5::resources::CodeSystemConcept) -> Self {
		let code = concept.code;
		let display = concept.display;
		let definition = concept.definition;

		Self { code, display, definition }
	}
}

/// Parse a Bundle into Codes.
pub fn parse_stu3(input: &str) -> Vec<Code> {
	let bundle: stu3::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing codes Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			stu3::resources::Resource::CodeSystem(code_system) => Some(code_system),
			_ => None,
		})
		.map(Code::from)
		.collect()
}

/// Parse a Bundle into Codes.
pub fn parse_r4b(input: &str) -> Vec<Code> {
	let bundle: r4b::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing codes Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r4b::resources::Resource::CodeSystem(code_system) => Some(code_system),
			_ => None,
		})
		.map(Code::from)
		.collect()
}

/// Parse a Bundle into Codes.
pub fn parse_r5(input: &str) -> Vec<Code> {
	let bundle: r5::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing codes Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r5::resources::Resource::CodeSystem(code_system) => Some(code_system),
			_ => None,
		})
		.map(Code::from)
		.collect()
}

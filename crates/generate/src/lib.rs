//! Code generation for FHIR types.
#![allow(clippy::expect_used, clippy::print_stdout)] // Just a generator crate.

mod generate;
mod model;
mod parse;

use std::fs;

use anyhow::{Context, Result};
use proc_macro2::TokenStream;

use crate::model::StructureDefinitionKind;

/// Generate code for a FHIR version. Must match the folder name for the input
/// data and the output folder name.
pub fn generate_code(version_folder: &str) -> Result<()> {
	let base_folder = env!("CARGO_MANIFEST_DIR");

	let codes_file =
		fs::read_to_string(format!("{base_folder}/definitions/{version_folder}/valuesets.json"))?;
	let codes = match version_folder {
		"r4b" => parse::codes::parse_r4b(&codes_file),
		"r5" => parse::codes::parse_r5(&codes_file),
		_ => panic!("Unrecognized version `{version_folder}`"),
	};
	let (generated_code, generated_codes) = generate::generate_codes(codes)?;
	fs::write(
		format!("{base_folder}/../fhir-model/src/{version_folder}/codes/generated.rs"),
		format_code(generated_code)?,
	)?;

	let types_file = fs::read_to_string(format!(
		"{base_folder}/definitions/{version_folder}/profiles-types.json"
	))?;
	let types = match version_folder {
		"r4b" => parse::structures::parse_r4b(&types_file),
		"r5" => parse::structures::parse_r5(&types_file),
		_ => panic!("Unrecognized version `{version_folder}`"),
	};
	let generated_code = generate::generate_types(types, &generated_codes)?;
	fs::write(
		format!("{base_folder}/../fhir-model/src/{version_folder}/types/generated.rs"),
		format_code(generated_code)?,
	)?;

	let resources_file = fs::read_to_string(format!(
		"{base_folder}/definitions/{version_folder}/profiles-resources.json"
	))?;
	let resources = match version_folder {
		"r4b" => parse::structures::parse_r4b(&resources_file),
		"r5" => parse::structures::parse_r5(&resources_file),
		_ => panic!("Unrecognized version `{version_folder}`"),
	};

	let identifiable = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.filter_map(|ty| {
			ty.elements
				.fields
				.iter()
				.any(|field| field.name() == "identifier" && field.is_array())
				.then_some(&ty.name)
		})
		.collect::<Vec<_>>();
	println!("Identifiable resources: {identifiable:?}");

	let generated_code = generate::generate_resources(resources, &generated_codes)?;
	fs::write(
		format!("{base_folder}/../fhir-model/src/{version_folder}/resources/generated.rs"),
		format_code(generated_code)?,
	)?;

	Ok(())
}

/// Convert a TokenStream to formatted Rust code.
fn format_code(code: TokenStream) -> Result<String> {
	let parsed = syn::parse2::<syn::File>(code).context("Parsing generated code to syn::File")?;
	Ok(prettyplease::unparse(&parsed))
}

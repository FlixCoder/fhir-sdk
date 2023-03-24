//! Code generation functionality.

mod gen_codes;
mod gen_types;

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
	codes::Code,
	structures::{Type, TypeKind},
};

/// Generate the Rust code for the FHIR codes.
pub fn generate_codes(mut codes: Vec<Code>) -> Result<(TokenStream, Vec<String>)> {
	let mut generated_codes = Vec::new();

	// Deduplicate and sort the codes..
	codes.sort_by_key(|code| code.name.clone());
	codes.dedup_by_key(|code| code.name.clone());

	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	let codes: Vec<TokenStream> = codes
		.into_iter()
		.filter(|code| code.name.is_pascal_case())
		.inspect(|code| generated_codes.push(code.name.clone()))
		.map(gen_codes::generate_code_enum)
		.collect::<Result<_, _>>()?;

	// Generate the code.
	let gen_enum = quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use serde::{Serialize, Deserialize};

		#(#codes)*
	};
	Ok((gen_enum, generated_codes))
}

/// Generate the Rust code for the FHIR types.
pub fn generate_types(types: Vec<Type>, generated_codes: &[String]) -> Result<TokenStream> {
	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	let types: Vec<TokenStream> = types
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::ComplexType)
		.map(|ty| gen_types::generate_type_struct(ty, generated_codes))
		.collect::<Result<_, _>>()?;

	// Generate the code.
	Ok(quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use ::core::num::NonZeroU32;
		use serde::{Serialize, Deserialize};
		use typed_builder::TypedBuilder;
		use super::super::codes;

		#(#types)*

		/// Extension of a field.
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
		#[serde(rename_all = "camelCase")]
		#[builder(field_defaults(setter(into)))]
		pub struct FieldExtension {
			/// Unique id for inter-element referencing
			#[serde(default, skip_serializing_if = "Option::is_none")]
			#[builder(default)]
			pub id: Option<String>,
			/// Additional content defined by implementations
			#[serde(default, skip_serializing_if = "Vec::is_empty")]
			#[builder(default)]
			pub extension: Vec<Extension>,
		}
	})
}

/// Generate the Rust code for the FHIR resources.
pub fn generate_resources(resources: Vec<Type>, generated_codes: &[String]) -> Result<TokenStream> {
	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	let resource_defs: Vec<TokenStream> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.map(|ty| gen_types::generate_type_struct(ty, generated_codes))
		.collect::<Result<_, _>>()?;
	let resource_names: Vec<_> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.map(|ty| &ty.name)
		.map(|name| format_ident!("{name}"))
		.collect();
	let resource_conversions = resource_conversion_impls(&resource_names);

	// Generate the code.
	Ok(quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use ::core::num::NonZeroU32;
		use serde::{Serialize, Deserialize};
		use typed_builder::TypedBuilder;
		use super::super::codes;
		use super::super::types::*;

		#(#resource_defs)*

		/// Generic resource holding any FHIR resources.
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(tag = "resourceType")]
		pub enum Resource {
			#(
				#[doc = stringify!(#resource_names)]
				#resource_names(#resource_names),
			)*
		}

		/// Resource type field of the FHIR resources.
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		pub enum ResourceType {
			#(
				#[doc = stringify!(#resource_names)]
				#resource_names,
			)*
		}

		/// Wrong resource type for conversion to the specified type.
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub struct WrongResourceType;
		impl ::core::fmt::Display for WrongResourceType {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				write!(f, "The Resource is of a different type than requested")
			}
		}
		impl ::std::error::Error for WrongResourceType {}

		#resource_conversions
	})
}

/// Conversion implementations between specific resources and the Resource enum.
fn resource_conversion_impls(names: &[Ident]) -> TokenStream {
	quote! {
		#(
			impl From<#names> for Resource {
				fn from(resource: #names) -> Self {
					Self::#names(resource)
				}
			}

			impl TryFrom<Resource> for #names {
				type Error = WrongResourceType;

				fn try_from(resource: Resource) -> Result<Self, Self::Error> {
					match resource {
						Resource::#names(r) => Ok(r),
						_ => Err(WrongResourceType),
					}
				}
			}

			impl<'a> TryFrom<&'a Resource> for &'a #names {
				type Error = WrongResourceType;

				fn try_from(resource: &'a Resource) -> Result<Self, Self::Error> {
					match resource {
						Resource::#names(r) => Ok(r),
						_ => Err(WrongResourceType),
					}
				}
			}

			impl<'a> TryFrom<&'a mut Resource> for &'a mut #names {
				type Error = WrongResourceType;

				fn try_from(resource: &'a mut Resource) -> Result<Self, Self::Error> {
					match resource {
						Resource::#names(r) => Ok(r),
						_ => Err(WrongResourceType),
					}
				}
			}
		)*
	}
}

/// Map field name to proper snake case identifier, with escaped rust keywords.
fn map_field_ident(name: &str) -> Ident {
	match name.to_snake_case().as_str() {
		"type" => format_ident!("r#type"),
		"abstract" => format_ident!("r#abstract"),
		"use" => format_ident!("r#use"),
		"ref" => format_ident!("r#ref"),
		"for" => format_ident!("r#for"),
		"mut" => format_ident!("r#mut"),
		name => format_ident!("{name}"),
	}
}

/// Map primitive type to Rust type.
fn map_type(ty: &str) -> Ident {
	match ty {
		"boolean" => format_ident!("bool"),
		"id" | "string" | "code" | "markdown" | "xhtml" => format_ident!("String"),
		"decimal" => format_ident!("f64"),
		"unsignedInt" => format_ident!("u32"),
		"integer" => format_ident!("i32"),
		"integer64" => format_ident!("i64"),
		"positiveInt" => format_ident!("NonZeroU32"),
		// TODO: turn the following types into proper rust types.
		"date" => format_ident!("String"),
		"dateTime" => format_ident!("String"),
		"instant" => format_ident!("String"),
		"time" => format_ident!("String"),
		"uri" => format_ident!("String"),
		"url" => format_ident!("String"),
		"canonical" => format_ident!("String"),
		"base64Binary" => format_ident!("String"),
		"oid" => format_ident!("String"),
		"uuid" => format_ident!("String"),
		_ => format_ident!("{ty}"),
	}
}

//! FHIR codes generation.

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::comments::sanitize;
use crate::model::codes::Code;

/// Generate a Rust enum for a FHIR code.
pub fn generate_code_enum(mut code: Code) -> Result<TokenStream> {
	let name = &code.name;
	let ident = format_ident!("{name}");

	let mut documentation = format!("**[{name}]({})**.", code.system);
	if let Some(description) = &code.description {
		documentation.push_str(&format!(" {description}"));
	}
	if let Some(version) = &code.version {
		documentation.push_str(&format!("\n\nFHIR version: {version}."));
	}

	let derive_copy = code.is_value_set.then_some(quote!(#[derive(Copy)]));

	code.items.sort_by_key(|item| item.code.clone());
	code.items.dedup_by_key(|item| item.code.clone());
	let variants = code.items.iter().map(|item| {
		let mut variant_doc = format!(" **{}**\n\n", item.code);
		if let Some(display) = &item.display {
			variant_doc.push_str(display);
			variant_doc.push_str(". ");
		}
		if let Some(description) = &item.definition {
			variant_doc.push_str(&sanitize(description));
			variant_doc.push(' ');
		}

		let variant = variant_ident(&item.code);

		quote! {
			#[doc = #variant_doc]
			#variant,
		}
	});

	let custom_variant = (!code.is_value_set).then_some(quote! {
		#[doc = " Custom code value."]
		_Custom(String),
	});

	let from_str_impl = from_str_impl(&ident, &code)?;
	let as_ref_impl = as_ref_impl(&ident, &code)?;
	let fmt_impls = fmt_impls(&ident)?;
	let deserialize_impl = deserialize_impl(&ident)?;
	let serialize_impl = serialize_impl(&ident)?;
	let convert_impls = convert_impls(&ident, &code)?;

	Ok(quote! {
		#[doc = #documentation]
		#[derive(PartialEq, Eq, Clone)]
		#derive_copy
		pub enum #ident {
			#(#variants)*
			#custom_variant
		}

		#from_str_impl
		#as_ref_impl
		#fmt_impls
		#deserialize_impl
		#serialize_impl
		#convert_impls
	})
}

/// Generate FromStr implementation for the FHIR code.
fn from_str_impl(ident: &Ident, code: &Code) -> Result<TokenStream> {
	let convert_lowercase = (!code.case_sensitive).then_some(quote!(.to_lowercase().as_str()));

	let branches = code.items.iter().map(|item| {
		let variant = variant_ident(&item.code);
		let code = if code.case_sensitive { item.code.clone() } else { item.code.to_lowercase() };
		quote!(#code => Ok(Self::#variant),)
	});

	let default_branch = if code.is_value_set {
		quote!(_ => Err(format!("Unknown value: {s}")))
	} else {
		quote!(_ => Ok(Self::_Custom(s.to_owned())))
	};

	Ok(quote! {
		impl ::core::str::FromStr for #ident {
			type Err = String;

			#[allow(clippy::match_single_binding)]
			fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
				match s #convert_lowercase {
					#(#branches)*
					#default_branch
				}
			}
		}
	})
}

/// Generate AsRef<str> implementation for the FHIR code.
fn as_ref_impl(ident: &Ident, code: &Code) -> Result<TokenStream> {
	let variants = code.items.iter().map(|item| {
		let code = &item.code;
		let variant = variant_ident(code);
		quote!(Self::#variant => #code,)
	});

	let custom_branch = (!code.is_value_set).then_some(quote!(Self::_Custom(s) => s.as_str(),));

	Ok(quote! {
		impl AsRef<str> for #ident {
			fn as_ref(&self) -> &str {
				match self {
					#(#variants)*
					#custom_branch
				}
			}
		}
	})
}

/// Generate Display implementation for the FHIR code.
fn fmt_impls(ident: &Ident) -> Result<TokenStream> {
	Ok(quote! {
		impl ::std::fmt::Debug for #ident {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				f.write_str(self.as_ref())
			}
		}

		impl ::std::fmt::Display for #ident {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				f.write_str(self.as_ref())
			}
		}
	})
}

/// Generate Deserialize implementation for the FHIR code.
fn deserialize_impl(ident: &Ident) -> Result<TokenStream> {
	Ok(quote! {
		impl<'de> Deserialize<'de> for #ident {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de> {
				use serde::de::Error;
				let string = String::deserialize(deserializer)?;
				string.parse().map_err(D::Error::custom)
			}
		}
	})
}

/// Generate Serialize implementation for the FHIR code.
fn serialize_impl(ident: &Ident) -> Result<TokenStream> {
	Ok(quote! {
		impl Serialize for #ident {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: serde::Serializer {
				self.as_ref().serialize(serializer)
			}
		}
	})
}

/// Convert implementations of the codes to Coding and CodeableConcept.
fn convert_impls(ident: &Ident, code: &Code) -> Result<TokenStream> {
	let system = &code.system;
	Ok(quote! {
		impl From<#ident> for Coding {
			fn from(code: #ident) -> Self {
				CodingInner {
					system: Some(#system.to_owned()),
					code: Some(code.as_ref().to_owned()),
					display: Some(format!("{code}")),
					id: None,
					extension: Vec::new(),
					system_ext: None,
					version: None,
					version_ext: None,
					code_ext: None,
					display_ext: None,
					user_selected: None,
					user_selected_ext: None,
				}.into()
			}
		}

		impl From<#ident> for CodeableConcept {
			fn from(code: #ident) -> Self {
				let text = format!("{code}");
				let coding = Coding::from(code);
				CodeableConceptInner {
					coding: vec![Some(coding)],
					text: Some(text),
					id: None,
					extension: Vec::new(),
					coding_ext: Vec::new(),
					text_ext: None,
				}.into()
			}
		}
	})
}

/// Generate an identifier for an enum variant safely.
fn variant_ident(s: &str) -> Ident {
	match s {
		"self" => format_ident!("_Self"),
		"<" => format_ident!("Less"),
		"<=" => format_ident!("LessOrEqual"),
		">" => format_ident!("Greater"),
		">=" => format_ident!("GreaterOrEqual"),
		"=" => format_ident!("Equal"),
		"!=" => format_ident!("NotEqual"),
		"+" => format_ident!("Plus"),
		"-" => format_ident!("Minus"),
		number if number.starts_with(char::is_numeric) => {
			format_ident!("N{}", number.to_pascal_case())
		}
		name => {
			format_ident!("{}", name.to_pascal_case())
		}
	}
}

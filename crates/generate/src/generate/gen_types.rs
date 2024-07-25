//! FHIR types generation.

use std::collections::HashMap;

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{comments::sanitize, map_field_ident, map_type};
use crate::model::{
	structures::{ChoiceField, CodeField, Field, ObjectField, StandardField, Type},
	StructureDefinitionKind,
};

/// Generate struct definition for a FHIR type.
pub fn generate_type_struct(
	ty: &Type,
	implemented_codes: &HashMap<String, String>,
) -> Result<TokenStream> {
	let name = &ty.name;
	let ident = format_ident!("{name}");
	let ident_inner = format_ident!("{name}Inner");
	let ident_builder = format_ident!("{name}Builder");
	let ident_builder_str = ident_builder.to_string();

	let mut doc_comment = format!(
		" {} \n\n **[{}]({}) v{}** \n\n {} \n\n {} \n\n ",
		sanitize(&ty.description),
		ty.name,
		ty.url,
		ty.version,
		sanitize(&ty.elements.short),
		sanitize(&ty.elements.definition)
	);
	if let Some(comment) = &ty.elements.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let resource_type_field = (ty.kind == StructureDefinitionKind::Resource).then(|| {
		let serde_default = format!("{name}::resource_type");
		let builder_default = format!("ResourceType::{ident}");
		quote! {
			/// Type of this FHIR resource.
			#[doc(hidden)]
			#[serde(default = #serde_default)]
			#[cfg_attr(feature = "builders", builder(default = #builder_default, setter(skip)))]
			resource_type: ResourceType,
		}
	});
	let resource_type_fn = (ty.kind == StructureDefinitionKind::Resource).then_some(quote! {
		impl #ident {
			/// Get the resource type for this FHIR resource.
			#[must_use]
			pub const fn resource_type() -> ResourceType {
				ResourceType::#ident
			}
		}
	});

	let (fields, structs): (Vec<_>, Vec<_>) = ty
		.elements
		.fields
		.iter()
		.map(|field| generate_field(field, &ident, ty, implemented_codes))
		.unzip();

	let wrapper_impls = wrapper_impls(&ident, &ident_inner, &ident_builder);

	Ok(quote! {
		#[doc = #doc_comment]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(transparent)]
		pub struct #ident(pub Box<#ident_inner>);

		#[doc = #doc_comment]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[cfg_attr(feature = "builders", derive(Builder))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "builders", builder(
			pattern = "owned",
			name = #ident_builder_str,
			build_fn(error = "crate::error::BuilderError", name = "build_inner"),
		))]
		pub struct #ident_inner {
			#resource_type_field
			#(#fields)*
		}

		#[cfg(feature = "builders")]
		impl #ident_builder {
			#[doc = concat!("Finalize building ", #name, ".")]
			pub fn build(self) -> Result<#ident, crate::error::BuilderError> {
				self.build_inner().map(Into::into)
			}
		}

		#wrapper_impls
		#resource_type_fn

		#(#structs)*
	})
}

/// Implementations of From, Deref and DerefMut towards the inner type.
fn wrapper_impls(ident: &Ident, ident_inner: &Ident, ident_builder: &Ident) -> TokenStream {
	quote! {
		impl From<#ident_inner> for #ident {
			fn from(inner: #ident_inner) -> Self {
				Self(Box::new(inner))
			}
		}

		impl ::core::ops::Deref for #ident {
			type Target = #ident_inner;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl ::core::ops::DerefMut for #ident {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		impl #ident {
			/// Start building an instance.
			#[cfg(feature = "builders")]
			#[must_use]
			pub fn builder() -> #ident_builder {
				#ident_builder ::default()
			}
		}
	}
}

/// Generate field information and sub-structs.
fn generate_field(
	field: &Field,
	type_ident: &Ident,
	base_type: &Type,
	implemented_codes: &HashMap<String, String>,
) -> (TokenStream, TokenStream) {
	let (doc_comment, (field_type, extension_type), structs) = match field {
		Field::Standard(f) => generate_standard_field(f),
		Field::Code(f) => generate_code_field(f, implemented_codes),
		Field::Choice(f) => generate_choice_field(f, type_ident),
		Field::Object(f) => generate_object_field(f, type_ident, base_type, implemented_codes),
	};

	let name = field.name().replace("[x]", "");
	let ident = map_field_ident(&name);
	let ty = construct_field_type(field, field_type);

	let serde_attr = field.optional().then(|| {
		if field.is_array() {
			quote!(#[serde(default, skip_serializing_if = "Vec::is_empty")])
		} else {
			quote!(#[serde(default, skip_serializing_if = "Option::is_none")])
		}
	});
	let builder_attr = field.optional().then_some(
		quote!(#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]),
	);
	let serde_rename_or_flatten = if matches!(field, Field::Choice(_)) {
		quote!(#[serde(flatten)])
	} else {
		quote!(#[serde(rename = #name)])
	};

	let extension_field = (!field.is_base_field()).then(|| {
		let ident_ext = format_ident!("{ident}_ext");
		let serde_ext = if matches!(field, Field::Choice(_)) {
			quote!(#[serde(flatten)])
		} else {
			let rename_ext = format!("_{name}");
			quote!(#[serde(rename = #rename_ext)])
		};

		if field.is_array() {
			quote! {
				/// Extension field.
				#[serde(default, skip_serializing_if = "Vec::is_empty")]
				#serde_ext
				#[cfg_attr(feature = "builders", builder(default))]
				pub #ident_ext: Vec<Option<#extension_type>>,
			}
		} else {
			quote! {
				/// Extension field.
				#[serde(default, skip_serializing_if = "Option::is_none")]
				#serde_ext
				#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
				pub #ident_ext: Option<#extension_type>,
			}
		}
	});

	let fields = quote! {
		#[doc = #doc_comment]
		#serde_attr
		#builder_attr
		#serde_rename_or_flatten
		pub #ident: #ty,
		#extension_field
	};
	(fields, structs)
}

/// Generate field information and sub-structs for a standard field.
fn generate_standard_field(field: &StandardField) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let mapped_type = map_type(&field.r#type);

	(doc_comment, (quote!(#mapped_type), format_ident!("FieldExtension")), quote!())
}

/// Generate field information and sub-structs for a code field.
fn generate_code_field(
	field: &CodeField,
	implemented_codes: &HashMap<String, String>,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment = format!(
		" **[{}]({}); {}** \n\n {} \n\n ",
		field.code_name.as_deref().unwrap_or_default(),
		field.code_url.as_deref().unwrap_or_default(),
		sanitize(&field.short),
		sanitize(&field.definition)
	);
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let mapped_type = code_field_type_name(field, implemented_codes);

	(doc_comment, (mapped_type, format_ident!("FieldExtension")), quote!())
}

/// Generate field information and sub-structs for a choice field.
fn generate_choice_field(
	field: &ChoiceField,
	type_ident: &Ident,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let enum_type = format_ident!("{type_ident}{}", field.name.replace("[x]", "").to_pascal_case());
	let enum_doc = format!(" Choice of types for the {} field in {type_ident}", field.name);

	let variants = field.types.iter().map(|ty| {
		let variant_ident = format_ident!("{}", ty.to_pascal_case());
		let variant_type = map_type(ty);
		let variant_doc = format!(" Variant accepting the {variant_ident} type.");
		let rename = field.name.replace("[x]", &variant_ident.to_string());

		quote! {
			#[doc = #variant_doc]
			#[serde(rename = #rename)]
			#variant_ident(#variant_type),
		}
	});

	let extension_type = format_ident!("{enum_type}Extension");
	let extension_doc = format!(" Extension value for {enum_type}.");
	let extension_variants = field.types.iter().map(|ty| {
		let variant_ident = format_ident!("{}", ty.to_pascal_case());
		let variant_doc = format!(" Variant accepting the {variant_ident} extension.");
		let rename = format!("_{}", field.name.replace("[x]", &variant_ident.to_string()));

		quote! {
			#[doc = #variant_doc]
			#[serde(rename = #rename)]
			#variant_ident(FieldExtension),
		}
	});

	let choice_enum = quote! {
		#[doc = #enum_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #enum_type {
			#(#variants)*
		}

		#[doc = #extension_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #extension_type {
			#(#extension_variants)*
		}
	};
	(doc_comment, (quote!(#enum_type), extension_type), choice_enum)
}

/// Generate field information and sub-structs for a object field.
fn generate_object_field(
	field: &ObjectField,
	type_ident: &Ident,
	base_type: &Type,
	implemented_codes: &HashMap<String, String>,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	if let Some(content_reference) = &field.content_reference {
		let field_type_name = content_reference.trim_start_matches('#').to_pascal_case();
		let ty = format_ident!("{field_type_name}");
		return (doc_comment, (quote!(#ty), format_ident!("FieldExtension")), quote!());
	}

	let struct_type = format_ident!("{type_ident}{}", field.name.to_pascal_case());

	let struct_doc = format!(" Sub-fields of the {} field in {type_ident}", field.name);

	let (fields, structs): (Vec<_>, Vec<_>) = field
		.fields
		.iter()
		.map(|f| generate_field(f, &struct_type, base_type, implemented_codes))
		.unzip();

	let object_struct_builder = format_ident!("{struct_type}Builder");
	let object_struct_builder_name = object_struct_builder.to_string();
	let object_struct = quote! {
		#[doc = #struct_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[cfg_attr(feature = "builders", derive(Builder))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "builders", builder(pattern = "owned", name = #object_struct_builder_name, build_fn(error = "crate::error::BuilderError")))]
		pub struct #struct_type {
			#(#fields)*
		}

		#[cfg(feature = "builders")]
		impl #struct_type {
			#[doc = "Start building a new instance"]
			#[must_use]
			pub fn builder() -> #object_struct_builder {
				#object_struct_builder ::default()
			}
		}
	};
	let structs = [object_struct]
		.into_iter()
		.chain(structs)
		.reduce(|mut l, r| {
			l.extend(r);
			l
		})
		.expect("Cannot fail");
	(doc_comment, (quote!(#struct_type), format_ident!("FieldExtension")), structs)
}

/// Construct the type of a field.
pub fn construct_field_type(field: &Field, field_type: TokenStream) -> TokenStream {
	if field.is_array() {
		if field.is_base_field() {
			quote!(Vec<#field_type>)
		} else {
			quote!(Vec<Option<#field_type>>)
		}
	} else if field.optional() {
		quote!(Option<#field_type>)
	} else {
		quote!(#field_type)
	}
}

/// Compute the type name of a CodeField.
pub fn code_field_type_name(
	field: &CodeField,
	implemented_codes: &HashMap<String, String>,
) -> TokenStream {
	let contains_name = field
		.code_name
		.as_ref()
		.map_or(false, |code_name| implemented_codes.values().any(|value| value == code_name));
	let contains_url =
		field.code_url.as_ref().map_or(false, |code_url| implemented_codes.contains_key(code_url));
	if field.r#type.as_str() == "code" && (contains_name || contains_url) {
		let type_name = field
			.code_url
			.as_ref()
			.and_then(|code_url| implemented_codes.get(code_url))
			.or(field.code_name.as_ref())
			.expect("Could not get FHIR code name to generate the field's type");
		let ty = format_ident!("{type_name}");
		quote!(codes::#ty)
	} else {
		let mapped_type = map_type(&field.r#type);
		quote!(#mapped_type)
	}
}

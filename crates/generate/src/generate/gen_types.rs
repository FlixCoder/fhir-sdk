//! FHIR types generation.

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{map_field_ident, map_type};
use crate::structures::{
	ChoiceField, CodeField, Field, ObjectField, StandardField, Type, TypeKind,
};

/// Generate struct definition for a FHIR type.
pub fn generate_type_struct(ty: &Type, implemented_codes: &[String]) -> Result<TokenStream> {
	let name = &ty.name;
	let ident = format_ident!("{name}");
	let ident_inner = format_ident!("{name}Inner");
	let ident_builder = format_ident!("{name}Builder");

	let mut doc_comment = format!(
		" # {name} \n\n {} \n\n ## {} (FHIR version: {}) \n\n {} \n\n {} \n\n ",
		ty.description.replace('\r', "\n"),
		ty.elements.name,
		ty.version,
		ty.elements.short.replace('\r', "\n"),
		ty.elements.definition.replace('\r', "\n")
	);
	if let Some(comment) = &ty.elements.comment {
		doc_comment.push_str(&comment.replace('\r', "\n"));
		doc_comment.push(' ');
	}

	let resource_type_field = (ty.kind == TypeKind::Resource).then(|| {
		let serde_default = format!("{name}::resource_type");
		quote! {
			/// Type of this FHIR resource.
			#[doc(hidden)]
			#[serde(default = #serde_default)]
			#[builder(default = ResourceType::#ident, setter(skip))]
			resource_type: ResourceType,
		}
	});
	let resource_type_fn = (ty.kind == TypeKind::Resource).then_some(quote! {
		impl #ident {
			/// Get the resource type for this FHIR resource.
			#[must_use]
			pub fn resource_type() -> ResourceType {
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
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
		#[serde(rename_all = "camelCase")]
		#[builder(
			builder_method(vis = ""),
			builder_type(name = #ident_builder),
			build_method(into = #ident),
			field_defaults(setter(into)),
		)]
		pub struct #ident_inner {
			#resource_type_field
			#(#fields)*
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
			pub fn builder() -> #ident_builder {
				#ident_inner ::builder()
			}
		}
	}
}

/// Generate field information and sub-structs.
fn generate_field(
	field: &Field,
	type_ident: &Ident,
	base_type: &Type,
	implemented_codes: &[String],
) -> (TokenStream, TokenStream) {
	let (doc_comment, field_type, structs) = match field {
		Field::Standard(f) => generate_standard_field(f),
		Field::Code(f) => generate_code_field(f, implemented_codes),
		Field::Choice(f) => generate_choice_field(f, type_ident),
		Field::Object(f) => generate_object_field(f, type_ident, base_type, implemented_codes),
	};

	let name = field.name().replace("[x]", "");
	let ident = map_field_ident(&name);
	let ty = if field.is_array() {
		if field.is_base_field() {
			quote!(Vec<#field_type>)
		} else {
			quote!(Vec<Option<#field_type>>)
		}
	} else if field.optional() {
		quote!(Option<#field_type>)
	} else {
		quote!(#field_type)
	};

	let serde_attr = field.optional().then(|| {
		if field.is_array() {
			quote!(#[serde(default, skip_serializing_if = "Vec::is_empty")])
		} else {
			quote!(#[serde(default, skip_serializing_if = "Option::is_none")])
		}
	});
	let builder_attr =
		field.optional().then_some(quote!(#[builder(default, setter(doc = #doc_comment))]));
	let serde_rename_or_flatten = if matches!(field, Field::Choice(_)) {
		quote!(#[serde(flatten)])
	} else {
		quote!(#[serde(rename = #name)])
	};

	let extension_field = (!field.is_base_field()).then(|| {
		let ident_ext = format_ident!("{ident}_ext");
		let rename_ext = format!("_{name}");

		if field.is_array() {
			quote! {
				/// Extension field.
				#[serde(rename = #rename_ext, default, skip_serializing_if = "Vec::is_empty")]
				#[builder(default, setter(doc = "Field extension."))]
				pub #ident_ext: Vec<Option<FieldExtension>>,
			}
		} else {
			quote! {
				/// Extension field.
				#[serde(rename = #rename_ext, default, skip_serializing_if = "Option::is_none")]
				#[builder(default, setter(doc = "Field extension."))]
				pub #ident_ext: Option<FieldExtension>,
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
fn generate_standard_field(field: &StandardField) -> (String, TokenStream, TokenStream) {
	let mut doc_comment = format!(
		" # {} \n\n {} \n\n ",
		field.short.replace('\r', "\n"),
		field.definition.replace('\r', "\n")
	);
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&comment.replace('\r', "\n"));
		doc_comment.push(' ');
	}

	let mapped_type = map_type(&field.r#type);

	(doc_comment, quote!(#mapped_type), quote!())
}

/// Generate field information and sub-structs for a code field.
fn generate_code_field(
	field: &CodeField,
	implemented_codes: &[String],
) -> (String, TokenStream, TokenStream) {
	let mut doc_comment = format!(
		" # {}; {} \n\n {} \n\n ",
		field.code,
		field.short.replace('\r', "\n"),
		field.definition.replace('\r', "\n")
	);
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&comment.replace('\r', "\n"));
		doc_comment.push(' ');
	}

	let mapped_type = if field.r#type.as_str() == "code" && implemented_codes.contains(&field.code)
	{
		let ty = format_ident!("{}", field.code);
		quote!(codes::#ty)
	} else {
		let mapped_type = map_type(&field.r#type);
		quote!(#mapped_type)
	};

	(doc_comment, mapped_type, quote!())
}

/// Generate field information and sub-structs for a choice field.
fn generate_choice_field(
	field: &ChoiceField,
	type_ident: &Ident,
) -> (String, TokenStream, TokenStream) {
	let mut doc_comment = format!(
		" # {} \n\n {} \n\n ",
		field.short.replace('\r', "\n"),
		field.definition.replace('\r', "\n")
	);
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&comment.replace('\r', "\n"));
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

	let choice_enum = quote! {
		#[doc = #enum_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #enum_type {
			#(#variants)*
		}
	};
	(doc_comment, quote!(#enum_type), choice_enum)
}

/// Generate field information and sub-structs for a object field.
fn generate_object_field(
	field: &ObjectField,
	type_ident: &Ident,
	base_type: &Type,
	implemented_codes: &[String],
) -> (String, TokenStream, TokenStream) {
	let mut doc_comment = format!(
		" # {} \n\n {} \n\n ",
		field.short.replace('\r', "\n"),
		field.definition.replace('\r', "\n")
	);
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&comment.replace('\r', "\n"));
		doc_comment.push(' ');
	}

	if let Some(content_reference) = &field.content_reference {
		let field_type_name = content_reference.trim_start_matches('#').to_pascal_case();
		let ty = format_ident!("{field_type_name}");
		return (doc_comment, quote!(#ty), quote!());
	}

	let struct_type = format_ident!("{type_ident}{}", field.name.to_pascal_case());

	let struct_doc = format!(" Sub-fields of the {} field in {type_ident}", field.name);

	let (fields, structs): (Vec<_>, Vec<_>) = field
		.fields
		.iter()
		.map(|f| generate_field(f, &struct_type, base_type, implemented_codes))
		.unzip();

	let object_struct = quote! {
		#[doc = #struct_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
		#[builder(field_defaults(setter(into)))]
		#[serde(rename_all = "camelCase")]
		pub struct #struct_type {
			#(#fields)*
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
	(doc_comment, quote!(#struct_type), structs)
}

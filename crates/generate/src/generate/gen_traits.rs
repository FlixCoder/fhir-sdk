//! Generate traits for base resource types.

use anyhow::{anyhow, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{
	gen_types::{code_field_type_name, construct_field_type},
	map_field_ident, map_type,
};
use crate::structures::{Field, Type, TypeKind};

/// Generate the BaseResource trait and its implementations.
pub fn generate_base_resource(
	resources: &[Type],
	implemented_codes: &[String],
) -> Result<TokenStream> {
	let resource = resources
		.iter()
		.filter(|ty| ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.find(|ty| &ty.name == "Resource")
		.ok_or(anyhow!("Could not find base Resource definition"))?;
	let (field_names, field_types) =
		get_field_names_and_types(&resource.elements.fields, implemented_codes);

	let ident = format_ident!("BaseResource");
	let trait_definition = make_trait_definition(resource, &field_names, &field_types, &ident);

	let trait_implementations: TokenStream = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.map(|ty| make_trait_implementation(ty, &field_names, &field_types, &ident))
		.collect();

	let filtered_resources: Vec<_> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.map(|ty| format_ident!("{}", ty.name))
		.collect();
	let impl_resource_as_trait = quote! {
		impl Resource {
			/// Return the resource as base resource.
			#[must_use]
			#[inline]
			pub fn as_base_resource(&self) -> &dyn #ident {
				match self {
					#(
						Self::#filtered_resources(r) => r,
					)*
				}
			}

			/// Return the resource as mutable base resource.
			#[must_use]
			#[inline]
			pub fn as_base_resource_mut(&mut self) -> &mut dyn #ident {
				match self {
					#(
						Self::#filtered_resources(r) => r,
					)*
				}
			}
		}
	};

	Ok(quote! {
		#trait_definition
		#trait_implementations
		#impl_resource_as_trait
	})
}

/// Generate the DomainResource trait and its implementations.
pub fn generate_domain_resource(
	resources: &[Type],
	implemented_codes: &[String],
) -> Result<TokenStream> {
	let resource = resources
		.iter()
		.filter(|ty| ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.find(|ty| &ty.name == "DomainResource")
		.ok_or(anyhow!("Could not find DomainResource definition"))?;
	let (field_names, field_types) =
		get_field_names_and_types(&resource.elements.fields, implemented_codes);

	let ident = format_ident!("DomainResource");
	let trait_definition = make_trait_definition(resource, &field_names, &field_types, &ident);

	let trait_implementations: TokenStream = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.filter(|ty| ty.base.as_ref().map_or(false, |base| base.ends_with("DomainResource")))
		.map(|ty| make_trait_implementation(ty, &field_names, &field_types, &ident))
		.collect();

	let filtered_resources: Vec<_> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.filter(|ty| ty.base.as_ref().map_or(false, |base| base.ends_with("DomainResource")))
		.map(|ty| format_ident!("{}", ty.name))
		.collect();
	let impl_resource_as_trait = quote! {
		impl Resource {
			/// Return the resource as domain resource.
			#[must_use]
			#[inline]
			pub fn as_domain_resource(&self) -> Option<&dyn #ident> {
				match self {
					#(
						Self::#filtered_resources(r) => Some(r),
					)*
					_ => None,
				}
			}

			/// Return the resource as mutable domain resource.
			#[must_use]
			#[inline]
			pub fn as_domain_resource_mut(&mut self) -> Option<&mut dyn #ident> {
				match self {
					#(
						Self::#filtered_resources(r) => Some(r),
					)*
					_ => None,
				}
			}
		}
	};

	Ok(quote! {
		#trait_definition
		#trait_implementations
		#impl_resource_as_trait
	})
}

/// Generate the NamedResource trait and its implementations.
pub fn generate_named_resource(resources: &[Type]) -> Result<TokenStream> {
	let trait_definition = quote! {
		/// Simple trait to supply (const) information about resources.
		pub trait NamedResource {
			/// The FHIR version of this resource.
			const FHIR_VERSION: &'static str;
			/// The ResourceType of this resouce.
			const TYPE: ResourceType;
		}
	};

	let trait_implementations: TokenStream = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == TypeKind::Resource)
		.map(|ty| {
			let name = format_ident!("{}", ty.name);
			let version = &ty.version;

			quote! {
				impl NamedResource for #name {
					const FHIR_VERSION: &'static str = #version;
					const TYPE: ResourceType = ResourceType::#name;
				}
			}
		})
		.collect();

	Ok(quote! {
		#trait_definition
		#trait_implementations
	})
}

/// Get field names and types from the elements.
fn get_field_names_and_types(
	fields: &[Field],
	implemented_codes: &[String],
) -> (Vec<Ident>, Vec<TokenStream>) {
	fields
		.iter()
		.cloned()
		.map(|mut field| {
			field.set_base_field();
			let field_name = map_field_ident(field.name());
			let field_type = match &field {
				Field::Standard(f) => {
					let ty = map_type(&f.r#type);
					quote!(#ty)
				}
				Field::Code(f) => code_field_type_name(f, implemented_codes),
				_ => panic!("Unsupported field type in BaseResource!"),
			};
			(field_name, construct_field_type(&field, field_type))
		})
		.unzip()
}

/// Make a trait definition from the resource, field names and types for the
/// given trait name.
fn make_trait_definition(
	resource: &Type,
	field_names: &[Ident],
	field_types: &[TokenStream],
	ident: &Ident,
) -> TokenStream {
	let mut doc_comment = format!(
		" # {} \n\n {} \n\n ## {} (FHIR version: {}) \n\n {} \n\n {} \n\n ",
		resource.name,
		resource.description.replace('\r', "\n"),
		resource.elements.name,
		resource.version,
		resource.elements.short.replace('\r', "\n"),
		resource.elements.definition.replace('\r', "\n")
	);
	if let Some(comment) = &resource.elements.comment {
		doc_comment.push_str(&comment.replace('\r', "\n"));
		doc_comment.push(' ');
	}

	let mut_getters = field_names.iter().map(|name| format_ident!("{name}_mut"));
	let setters = field_names.iter().map(|name| format_ident!("set_{name}"));

	quote! {
		#[doc = #doc_comment]
		pub trait #ident {
			#(
				#[doc = concat!("Get `", stringify!(#field_names), "`.")]
				fn #field_names(&self) -> & #field_types;
				#[doc = concat!("Get `", stringify!(#field_names), "` mutably.")]
				fn #mut_getters(&mut self) -> &mut #field_types;
				#[doc = concat!("Set `", stringify!(#field_names), "`.")]
				fn #setters(&mut self, value: #field_types);
			)*
		}
	}
}

/// Make a trait implementation for the resource, given the field names and
/// types for the given trait name.
fn make_trait_implementation(
	resource: &Type,
	field_names: &[Ident],
	field_types: &[TokenStream],
	ident: &Ident,
) -> TokenStream {
	let name = format_ident!("{}", resource.name);
	let mut_getters = field_names.iter().map(|name| format_ident!("{name}_mut"));
	let setters = field_names.iter().map(|name| format_ident!("set_{name}"));

	quote! {
		impl #ident for #name {
			#(
				#[inline]
				fn #field_names(&self) -> & #field_types {
					&self.#field_names
				}

				#[inline]
				fn #mut_getters(&mut self) -> &mut #field_types {
					&mut self.#field_names
				}

				#[inline]
				fn #setters(&mut self, value: #field_types) {
					self.#field_names = value;
				}
			)*
		}
	}
}

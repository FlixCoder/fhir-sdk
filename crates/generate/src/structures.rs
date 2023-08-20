//! FHIR type definition parsing.
#![allow(clippy::fallible_impl_from)] // We want to panic on unexpected formats!

use std::collections::BTreeMap;

use fhir_model::r4b::{
	codes::{PublicationStatus, StructureDefinitionKind},
	resources::{Bundle, Resource, StructureDefinition, StructureDefinitionSnapshot},
	types::{ElementDefinition, ElementDefinitionType, ExtensionValue},
};

/// Type definition.
#[derive(Debug)]
pub struct Type {
	/// Name of the type.
	pub name: String,
	/// FHIR version.
	pub version: String,
	/// Description.
	pub description: String,
	/// Kind.
	pub kind: StructureDefinitionKind,
	/// Whether this is an abstract type.
	pub r#abstract: bool,
	/// Base definition.
	pub base: Option<String>,
	/// Status of the definition.
	pub status: PublicationStatus,
	/// Whether it is experimental.
	pub experimental: bool,
	/// Type of this type.
	pub r#type: String,
	/// Elements.
	pub elements: ObjectField,
}

impl From<StructureDefinition> for Type {
	fn from(structure_definition: StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let name = structure_definition.name;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion"),
			version
		);
		let description =
			structure_definition.description.expect("StructureDefinition.description");
		let kind = structure_definition.kind;
		let r#abstract = structure_definition.r#abstract;
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});
		let status = structure_definition.status;
		let experimental =
			structure_definition.experimental.expect("StructureDefinition.experimental");
		let r#type = structure_definition.r#type;
		let elements =
			ObjectField::from(structure_definition.snapshot.expect("StructureDefinition.snapshot"));

		Self {
			name,
			version,
			description,
			kind,
			r#abstract,
			base,
			status,
			experimental,
			r#type,
			elements,
		}
	}
}

/// Generic field.
#[derive(Debug, Clone)]
pub enum Field {
	/// Object that contains sub fields.
	Object(ObjectField),
	/// Definition of a normal field.
	Standard(StandardField),
	/// Definition of a code field.
	Code(CodeField),
	/// Definition of a choice field.
	Choice(ChoiceField),
}

impl Field {
	/// Add a field to the sub-fields. Does only work on object (and standard
	/// fields, by transforming them to an object).
	fn add_field(&mut self, remaining_path: &str, field: Field) {
		match self {
			Self::Object(object) => object.add_field(remaining_path, field),
			Self::Standard(standard) => {
				let mut object = ObjectField::from_standard(standard);
				object.add_field(remaining_path, field);
				*self = Self::Object(object);
			}
			_ => panic!("Tried to add field to non-object field!"),
		}
	}

	/// Resolve to a specific inner field. Does only work on object fields.
	#[allow(dead_code)] // Unused, because type_name is unused.
	pub fn resolve(&self, remaining_path: &str) -> &Field {
		match self {
			Self::Object(object) => object.resolve(remaining_path),
			_ => panic!("Tried to resolve into non-object field!"),
		}
	}

	/// Get the common field `name`.
	pub fn name(&self) -> &String {
		match self {
			Self::Standard(f) => &f.name,
			Self::Code(f) => &f.name,
			Self::Choice(f) => &f.name,
			Self::Object(f) => &f.name,
		}
	}

	/// Get the common field `optional`.
	pub fn optional(&self) -> bool {
		match self {
			Self::Standard(f) => f.optional,
			Self::Code(f) => f.optional,
			Self::Choice(f) => f.optional,
			Self::Object(f) => f.optional,
		}
	}

	/// Get the common field `is_array`.
	pub fn is_array(&self) -> bool {
		match self {
			Self::Standard(f) => f.is_array,
			Self::Code(f) => f.is_array,
			Self::Choice(f) => f.is_array,
			Self::Object(f) => f.is_array,
		}
	}

	/// Get the common field `is_base_field`.
	pub fn is_base_field(&self) -> bool {
		match self {
			Self::Standard(f) => f.is_base_field,
			Self::Code(f) => f.is_base_field,
			Self::Choice(f) => f.is_base_field,
			Self::Object(f) => f.is_base_field,
		}
	}

	/// Set the field to be a base field.
	pub fn set_base_field(&mut self) {
		match self {
			Self::Standard(f) => f.is_base_field = true,
			Self::Code(f) => f.is_base_field = true,
			Self::Choice(f) => f.is_base_field = true,
			Self::Object(f) => f.is_base_field = true,
		}
	}
}

impl From<ElementDefinition> for Field {
	fn from(element: ElementDefinition) -> Self {
		if element.path.ends_with("[x]") || element.r#type.len() > 1 {
			Self::Choice(ChoiceField::from(element))
		} else if element.binding.is_some() {
			Self::Code(CodeField::from(element))
		} else if element
			.extension
			.iter()
			.any(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			}) || element.content_reference.is_some()
		{
			Self::Object(ObjectField::from(element))
		} else {
			Self::Standard(StandardField::from(element))
		}
	}
}

/// Object that contains sub-fields.
#[derive(Debug, Clone)]
pub struct ObjectField {
	/// Field name.
	pub name: String,
	/// Short definition.
	pub short: String,
	/// Text definition.
	pub definition: String,
	/// Comment.
	pub comment: Option<String>,
	/// Whether this field is optional
	pub optional: bool,
	/// Whether this field is an array.
	pub is_array: bool,
	/// Whether this is a base type field.
	pub is_base_field: bool,
	/// Base type.
	pub r#type: Option<String>,
	/// Type name of this object.
	pub type_name: Option<String>,
	/// Content reference to re-use a type definition from somewhere else in
	/// this type.
	pub content_reference: Option<String>,
	/// Whether this field is a modifier field.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	pub is_summary: bool,

	/// Sub-fields.
	pub fields: Vec<Field>,
	/// Map from field name to field index.
	pub field_map: BTreeMap<String, usize>,
}

impl ObjectField {
	/// Add a field to the sub-fields.
	fn add_field(&mut self, remaining_path: &str, field: Field) {
		if let Some((name, remaining)) = remaining_path.split_once('.') {
			let object = self
				.fields
				.get_mut(self.field_map[name])
				.expect("Accessing already existing object fields");
			object.add_field(remaining, field);
		} else {
			assert!(!self.field_map.contains_key(remaining_path));
			self.fields.push(field);
			self.field_map.insert(remaining_path.to_owned(), self.fields.len() - 1);
		}
	}

	/// Resolve to a specific inner field.
	#[allow(dead_code)] // Unused, because type_name is unused.
	pub fn resolve(&self, remaining_path: &str) -> &Field {
		if let Some((name, remaining)) = remaining_path.split_once('.') {
			self.fields[self.field_map[name]].resolve(remaining)
		} else {
			&self.fields[self.field_map[remaining_path]]
		}
	}

	/// Construct object field from standard field.
	fn from_standard(field: &StandardField) -> Self {
		Self {
			name: field.name.clone(),
			short: field.short.clone(),
			definition: field.definition.clone(),
			comment: field.comment.clone(),
			optional: field.optional,
			is_array: field.is_array,
			is_base_field: field.is_base_field,
			r#type: Some(field.r#type.clone()),
			type_name: None,
			content_reference: None,
			is_modifier: field.is_modifier,
			is_summary: field.is_summary,
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		}
	}
}

impl From<StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: StructureDefinitionSnapshot) -> Self {
		let mut elements = snapshot.element.into_iter().flatten();
		let first = elements.next().expect("Found no ElementDefinition").0;
		let name = first.path;
		assert!(!name.contains('.'));
		let short = first.short.expect("ElementDefinition.short");
		let definition = first.definition.expect("ElementDefinition.definition");
		let comment = first.comment;
		let min = first.min.expect("ElementDefinition.min");
		let optional = min == 0;
		let max = first.max.expect("ElementDefinition.max");
		let is_array = &max != "1";
		let r#type = first.r#type.into_iter().flatten().next().map(type_to_string);
		let is_modifier = first.is_modifier.expect("ElementDefinition.isModifier");
		let is_summary = first.is_summary.unwrap_or(false);

		let fields = Vec::new();
		let field_map = BTreeMap::new();

		let mut object = Self {
			name,
			short,
			definition,
			comment,
			optional,
			is_array,
			is_base_field: false,
			r#type,
			type_name: None,
			content_reference: None,
			is_modifier,
			is_summary,
			fields,
			field_map,
		};

		for element in elements {
			let path = element.path.clone();
			let Some((top_name, remaining)) = path.split_once('.') else {
				panic!("Multiple top-level fields defined?");
			};
			assert_eq!(top_name, &object.name);

			let field = Field::from(element);
			object.add_field(remaining, field);
		}

		object
	}
}

impl From<ElementDefinition> for ObjectField {
	fn from(element: ElementDefinition) -> Self {
		let element = element.0;
		let name =
			element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned());
		let short = element.short.expect("ElementDefinition.short");
		let definition = element.definition.expect("ElementDefinition.definition");
		let comment = element.comment;
		let min = element.min.expect("ElementDefinition.min");
		let optional = min == 0;
		let max = element.max.expect("ElementDefinition.max");
		let is_array = &max != "1";
		let is_base_field = element.base.map_or(false, |base| base.path != element.path);
		let r#type = element.r#type.into_iter().flatten().next().map(type_to_string);
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				ExtensionValue::String(s) => s,
				_ => panic!("Wrong value type in ElemenentDefinition.extension"),
			});
		let content_reference = element.content_reference;
		let is_modifier = element.is_modifier.expect("ElementDefinition.isModifier");
		let is_summary = element.is_summary.unwrap_or(false);

		let fields = Vec::new();
		let field_map = BTreeMap::new();

		Self {
			name,
			short,
			definition,
			comment,
			optional,
			is_array,
			is_base_field,
			r#type,
			type_name,
			content_reference,
			is_modifier,
			is_summary,
			fields,
			field_map,
		}
	}
}

/// Definition of a normal field.
#[derive(Debug, Clone)]
pub struct StandardField {
	/// Field name.
	pub name: String,
	/// Short definition.
	pub short: String,
	/// Text definition.
	pub definition: String,
	/// Comment.
	pub comment: Option<String>,
	/// Whether this field is optional
	pub optional: bool,
	/// Whether this field is an array.
	pub is_array: bool,
	/// Whether this is a base type field.
	pub is_base_field: bool,
	/// Name of the type.
	pub r#type: String,
	/// Whether this field is a modifier field.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	pub is_summary: bool,
}

impl From<ElementDefinition> for StandardField {
	fn from(element: ElementDefinition) -> Self {
		let element = element.0;
		if element.r#type.is_empty() {
			panic!("Element without type: {element:#?}");
		}

		let name =
			element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned());
		let short = element.short.expect("ElementDefinition.short");
		let definition = element.definition.expect("ElementDefinition.definition");
		let comment = element.comment;
		let min = element.min.expect("ElementDefinition.min");
		let optional = min == 0;
		let max = element.max.expect("ElementDefinition.max");
		let is_array = &max != "1";
		let is_base_field = element.base.map_or(false, |base| base.path != element.path)
			|| element
				.r#type
				.first()
				.and_then(Option::as_ref)
				.map_or(false, |ty| &ty.code == "http://hl7.org/fhirpath/System.String");
		let r#type = element
			.r#type
			.into_iter()
			.flatten()
			.next()
			.map(type_to_string)
			.expect("ElementDefinition.type");
		let is_modifier = element.is_modifier.expect("ElementDefinition.isModifier");
		let is_summary = element.is_summary.unwrap_or(false);

		Self {
			name,
			short,
			definition,
			comment,
			optional,
			is_array,
			is_base_field,
			r#type,
			is_modifier,
			is_summary,
		}
	}
}

/// Definition of a code field.
#[derive(Debug, Clone)]
pub struct CodeField {
	/// Field name.
	pub name: String,
	/// Short definition.
	pub short: String,
	/// Text definition.
	pub definition: String,
	/// Comment.
	pub comment: Option<String>,
	/// Whether this field is optional
	pub optional: bool,
	/// Whether this field is an array.
	pub is_array: bool,
	/// Whether this is a base type field.
	pub is_base_field: bool,
	/// Name of the type.
	pub r#type: String,
	/// Name of the code.
	pub code: String,
	/// Whether this field is a modifier field.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	pub is_summary: bool,
}

impl From<ElementDefinition> for CodeField {
	fn from(element: ElementDefinition) -> Self {
		let element = element.0;
		let name =
			element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned());
		let short = element.short.expect("ElementDefinition.short");
		let definition = element.definition.expect("ElementDefinition.definition");
		let comment = element.comment;
		let min = element.min.expect("ElementDefinition.min");
		let optional = min == 0;
		let max = element.max.expect("ElementDefinition.max");
		let is_array = &max != "1";
		let is_base_field = element.base.map_or(false, |base| base.path != element.path);
		let r#type = element
			.r#type
			.into_iter()
			.flatten()
			.next()
			.map(type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			})
			.or(binding.value_set)
			.expect(
				"ElementDefinition.binding.extension.value or ElementDefinition.binding.valueSet",
			);
		let is_modifier = element.is_modifier.expect("ElementDefinition.isModifier");
		let is_summary = element.is_summary.unwrap_or(false);

		Self {
			name,
			short,
			definition,
			comment,
			optional,
			is_array,
			is_base_field,
			r#type,
			code,
			is_modifier,
			is_summary,
		}
	}
}

/// Definition of a choice field.
#[derive(Debug, Clone)]
pub struct ChoiceField {
	/// Field name.
	pub name: String,
	/// Short definition.
	pub short: String,
	/// Text definition.
	pub definition: String,
	/// Comment.
	pub comment: Option<String>,
	/// Whether this field is optional
	pub optional: bool,
	/// Whether this field is an array.
	pub is_array: bool,
	/// Whether this is a base type field.
	pub is_base_field: bool,
	/// Name of the possible types.
	pub types: Vec<String>,
	/// Whether this field is a modifier field.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	pub is_summary: bool,
}

impl From<ElementDefinition> for ChoiceField {
	fn from(element: ElementDefinition) -> Self {
		let element = element.0;
		let name =
			element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned());
		let short = element.short.expect("ElementDefinition.short");
		let definition = element.definition.expect("ElementDefinition.definition");
		let comment = element.comment;
		let min = element.min.expect("ElementDefinition.min");
		let optional = min == 0;
		let max = element.max.expect("ElementDefinition.max");
		let is_array = &max != "1";
		let is_base_field = element.base.map_or(false, |base| base.path != element.path);
		let types = element.r#type.into_iter().flatten().map(type_to_string).collect();
		let is_modifier = element.is_modifier.expect("ElementDefinition.isModifier");
		let is_summary = element.is_summary.unwrap_or(false);

		Self {
			name,
			short,
			definition,
			comment,
			optional,
			is_array,
			is_base_field,
			types,
			is_modifier,
			is_summary,
		}
	}
}

/// Convert a type value to a proper string of the type name.
fn type_to_string(r#type: ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.map(|v| match v {
						ExtensionValue::Url(url) => url,
						_ => panic!("ElementDefinition.type.extension.value is not URL"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code
}

/// Parse a Bundle into Types.
pub fn parse(input: &str) -> Vec<Type> {
	let bundle: Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			Resource::StructureDefinition(structure_definition) => Some(structure_definition),
			_ => None,
		})
		.map(Type::from)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_types_from_structure_definitions() {
		let included_types = include_str!("../definitions/r4b/profiles-types.json");
		parse(included_types);
		let included_resources = include_str!("../definitions/r4b/profiles-resources.json");
		parse(included_resources);

		let included_types = include_str!("../definitions/r5/profiles-types.json");
		parse(included_types);
		let included_resources = include_str!("../definitions/r5/profiles-resources.json");
		parse(included_resources);
	}
}

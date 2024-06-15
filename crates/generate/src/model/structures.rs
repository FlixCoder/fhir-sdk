//! Structures definitions.

use std::collections::BTreeMap;

use super::{PublicationStatus, StructureDefinitionKind};

/// Type definition.
#[derive(Debug)]
pub struct Type {
	/// Name of the type.
	pub name: String,
	/// FHIR version.
	pub version: String,
	/// URL of this type.
	pub url: String,
	/// Description.
	pub description: String,
	/// Kind.
	pub kind: StructureDefinitionKind,
	/// Whether this is an abstract type.
	pub r#abstract: bool,
	/// Base definition.
	pub base: Option<String>,
	/// Status of the definition.
	#[allow(dead_code)] // Might be used later.
	pub status: PublicationStatus,
	/// Whether it is experimental.
	#[allow(dead_code)] // Might be used later.
	pub experimental: bool,
	/// Type of this type.
	#[allow(dead_code)] // Might be used later.
	pub r#type: String,
	/// Elements.
	pub elements: ObjectField,
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
	pub fn add_field(&mut self, remaining_path: &str, field: Field) {
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
	#[allow(dead_code)] // Might be used later.
	pub r#type: Option<String>,
	/// Type name of this object.
	#[allow(dead_code)] // Might be used later.
	pub type_name: Option<String>,
	/// Content reference to re-use a type definition from somewhere else in
	/// this type.
	pub content_reference: Option<String>,
	/// Whether this field is a modifier field.
	#[allow(dead_code)] // Might be used later.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	#[allow(dead_code)] // Might be used later.
	pub is_summary: bool,

	/// Sub-fields.
	pub fields: Vec<Field>,
	/// Map from field name to field index.
	pub field_map: BTreeMap<String, usize>,
}

impl ObjectField {
	/// Add a field to the sub-fields.
	pub fn add_field(&mut self, remaining_path: &str, field: Field) {
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
	pub code_name: Option<String>,
	/// ValueSet URL of the code.
	pub code_url: Option<String>,
	/// Whether this field is a modifier field.
	#[allow(dead_code)] // Might be used later.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	#[allow(dead_code)] // Might be used later.
	pub is_summary: bool,
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
	#[allow(dead_code)] // Might be used later.
	pub is_modifier: bool,
	/// Whether this field is part of the summary.
	#[allow(dead_code)] // Might be used later.
	pub is_summary: bool,
}

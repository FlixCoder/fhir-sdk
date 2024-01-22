//! Structures parsing.

use std::collections::BTreeMap;

use fhir_model::{r4b, r5, stu3};

use crate::model::structures::{ChoiceField, CodeField, Field, ObjectField, StandardField, Type};

impl From<stu3::resources::StructureDefinition> for Type {
	fn from(structure_definition: stu3::resources::StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let name = structure_definition.name;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion").to_string(),
			version
		);
		let url = structure_definition.url;
		let description =
			structure_definition.description.expect("StructureDefinition.description");
		let kind = structure_definition.kind.into();
		let r#abstract = structure_definition.r#abstract;
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});
		let status = structure_definition.status.into();
		let experimental =
			structure_definition.experimental.expect("StructureDefinition.experimental");
		let r#type = structure_definition.r#type;
		let elements =
			ObjectField::from(structure_definition.snapshot.expect("StructureDefinition.snapshot"));

		Self {
			name,
			version,
			url,
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

impl From<r4b::resources::StructureDefinition> for Type {
	fn from(structure_definition: r4b::resources::StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let name = structure_definition.name;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion").to_string(),
			version
		);
		let url = structure_definition.url;
		let description =
			structure_definition.description.expect("StructureDefinition.description");
		let kind = structure_definition.kind.into();
		let r#abstract = structure_definition.r#abstract;
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});
		let status = structure_definition.status.into();
		let experimental =
			structure_definition.experimental.expect("StructureDefinition.experimental");
		let r#type = structure_definition.r#type;
		let elements =
			ObjectField::from(structure_definition.snapshot.expect("StructureDefinition.snapshot"));

		Self {
			name,
			version,
			url,
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

impl From<r5::resources::StructureDefinition> for Type {
	fn from(structure_definition: r5::resources::StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let name = structure_definition.name;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion").to_string(),
			version
		);
		let url = structure_definition.url;
		let description =
			structure_definition.description.expect("StructureDefinition.description");
		let kind = structure_definition.kind.into();
		let r#abstract = structure_definition.r#abstract;
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});
		let status = structure_definition.status.into();
		let experimental =
			structure_definition.experimental.expect("StructureDefinition.experimental");
		let r#type = structure_definition.r#type;
		let elements =
			ObjectField::from(structure_definition.snapshot.expect("StructureDefinition.snapshot"));

		Self {
			name,
			version,
			url,
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

impl From<stu3::types::ElementDefinition> for Field {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		if element.path.ends_with("[x]") {
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

impl From<r4b::types::ElementDefinition> for Field {
	fn from(element: r4b::types::ElementDefinition) -> Self {
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

impl From<r5::types::ElementDefinition> for Field {
	fn from(element: r5::types::ElementDefinition) -> Self {
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

impl From<stu3::resources::StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: stu3::resources::StructureDefinitionSnapshot) -> Self {
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
		let r#type = first.r#type.into_iter().flatten().next().map(stu3_type_to_string);
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

impl From<r4b::resources::StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: r4b::resources::StructureDefinitionSnapshot) -> Self {
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
		let r#type = first.r#type.into_iter().flatten().next().map(r4b_type_to_string);
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

impl From<r5::resources::StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: r5::resources::StructureDefinitionSnapshot) -> Self {
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
		let r#type = first.r#type.into_iter().flatten().next().map(r5_type_to_string);
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

impl From<stu3::types::ElementDefinition> for ObjectField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
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
		let r#type = element.r#type.into_iter().flatten().next().map(stu3_type_to_string);
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				stu3::types::ExtensionValue::String(s) => s,
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

impl From<r4b::types::ElementDefinition> for ObjectField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
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
		let r#type = element.r#type.into_iter().flatten().next().map(r4b_type_to_string);
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r4b::types::ExtensionValue::String(s) => s,
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

impl From<r5::types::ElementDefinition> for ObjectField {
	fn from(element: r5::types::ElementDefinition) -> Self {
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
		let r#type = element.r#type.into_iter().flatten().next().map(r5_type_to_string);
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r5::types::ExtensionValue::String(s) => s,
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

impl From<stu3::types::ElementDefinition> for StandardField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
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
			.map(stu3_type_to_string)
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

impl From<r4b::types::ElementDefinition> for StandardField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
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
			.map(r4b_type_to_string)
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

impl From<r5::types::ElementDefinition> for StandardField {
	fn from(element: r5::types::ElementDefinition) -> Self {
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
			.map(r5_type_to_string)
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

impl From<stu3::types::ElementDefinition> for CodeField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
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
			.map(stu3_type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code_name = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				stu3::types::ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			});
		// Remove version string at the end (|5.0.0).
		let code_url = binding
			.value_set
			.map(|code_url| match code_url {
				stu3::types::ElementDefinitionBindingValueSet::Uri(u) => u,
				stu3::types::ElementDefinitionBindingValueSet::Reference(r) => {
					r.reference.clone().expect("ElementDefinition.valueSetReference.reference")
				}
			})
			.map(|code_url| {
				code_url.split_once('|').map_or(code_url.as_str(), |(start, _end)| start).to_owned()
			});
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
			code_name,
			code_url,
			is_modifier,
			is_summary,
		}
	}
}

impl From<r4b::types::ElementDefinition> for CodeField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
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
			.map(r4b_type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code_name = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r4b::types::ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			});
		// Remove version string at the end (|5.0.0).
		let code_url = binding.value_set.map(|code_url| {
			code_url.split_once('|').map_or(code_url.as_str(), |(start, _end)| start).to_owned()
		});
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
			code_name,
			code_url,
			is_modifier,
			is_summary,
		}
	}
}

impl From<r5::types::ElementDefinition> for CodeField {
	fn from(element: r5::types::ElementDefinition) -> Self {
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
			.map(r5_type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code_name = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r5::types::ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			});
		// Remove version string at the end (|5.0.0).
		let code_url = binding.value_set.map(|code_url| {
			code_url.split_once('|').map_or(code_url.as_str(), |(start, _end)| start).to_owned()
		});
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
			code_name,
			code_url,
			is_modifier,
			is_summary,
		}
	}
}

impl From<stu3::types::ElementDefinition> for ChoiceField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
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
		let types = element.r#type.into_iter().flatten().map(stu3_type_to_string).collect();
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

impl From<r4b::types::ElementDefinition> for ChoiceField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
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
		let types = element.r#type.into_iter().flatten().map(r4b_type_to_string).collect();
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

impl From<r5::types::ElementDefinition> for ChoiceField {
	fn from(element: r5::types::ElementDefinition) -> Self {
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
		let types = element.r#type.into_iter().flatten().map(r5_type_to_string).collect();
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

fn stu3_type_to_string(r#type: stu3::types::ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.map(|v| match v {
						stu3::types::ExtensionValue::Uri(uri) => uri,
						_ => panic!("ElementDefinition.type.extension.value is not URI"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code
}

/// Convert a type value to a proper string of the type name.
fn r4b_type_to_string(r#type: r4b::types::ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.map(|v| match v {
						r4b::types::ExtensionValue::Url(url) => url,
						_ => panic!("ElementDefinition.type.extension.value is not URL"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code
}

/// Convert a type value to a proper string of the type name.
fn r5_type_to_string(r#type: r5::types::ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.map(|v| match v {
						r5::types::ExtensionValue::Url(url) => url,
						_ => panic!("ElementDefinition.type.extension.value is not URL"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code
}

/// Parse a Bundle into Types.
pub fn parse_stu3(input: &str) -> Vec<Type> {
	let bundle: stu3::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			stu3::resources::Resource::StructureDefinition(structure_definition) => {
				Some(structure_definition)
			}
			_ => None,
		})
		.map(Type::from)
		.collect()
}

/// Parse a Bundle into Types.
pub fn parse_r4b(input: &str) -> Vec<Type> {
	let bundle: r4b::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r4b::resources::Resource::StructureDefinition(structure_definition) => {
				Some(structure_definition)
			}
			_ => None,
		})
		.map(Type::from)
		.collect()
}

/// Parse a Bundle into Types.
pub fn parse_r5(input: &str) -> Vec<Type> {
	let bundle: r5::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r5::resources::Resource::StructureDefinition(structure_definition) => {
				Some(structure_definition)
			}
			_ => None,
		})
		.map(Type::from)
		.collect()
}

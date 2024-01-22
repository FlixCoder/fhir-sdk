//! Parsing of the StructureDefinitions into the common model.
#![allow(clippy::fallible_impl_from)] // We want to panic on unexpected formats!

pub mod codes;
pub mod structures;

use fhir_model::{r4b, r5, stu3};

use crate::model::{CodeSystemContentMode, PublicationStatus, StructureDefinitionKind};

impl From<stu3::codes::PublicationStatus> for PublicationStatus {
	fn from(value: stu3::codes::PublicationStatus) -> Self {
		match value {
			stu3::codes::PublicationStatus::Active => Self::Active,
			stu3::codes::PublicationStatus::Draft => Self::Draft,
			stu3::codes::PublicationStatus::Retired => Self::Retired,
			stu3::codes::PublicationStatus::Unknown => Self::Unknown,
		}
	}
}

impl From<r4b::codes::PublicationStatus> for PublicationStatus {
	fn from(value: r4b::codes::PublicationStatus) -> Self {
		match value {
			r4b::codes::PublicationStatus::Active => Self::Active,
			r4b::codes::PublicationStatus::Draft => Self::Draft,
			r4b::codes::PublicationStatus::Retired => Self::Retired,
			r4b::codes::PublicationStatus::Unknown => Self::Unknown,
		}
	}
}

impl From<r5::codes::PublicationStatus> for PublicationStatus {
	fn from(value: r5::codes::PublicationStatus) -> Self {
		match value {
			r5::codes::PublicationStatus::Active => Self::Active,
			r5::codes::PublicationStatus::Draft => Self::Draft,
			r5::codes::PublicationStatus::Retired => Self::Retired,
			r5::codes::PublicationStatus::Unknown => Self::Unknown,
		}
	}
}

impl From<stu3::codes::StructureDefinitionKind> for StructureDefinitionKind {
	fn from(value: stu3::codes::StructureDefinitionKind) -> Self {
		match value {
			stu3::codes::StructureDefinitionKind::ComplexType => Self::ComplexType,
			stu3::codes::StructureDefinitionKind::Logical => Self::Logical,
			stu3::codes::StructureDefinitionKind::PrimitiveType => Self::PrimitiveType,
			stu3::codes::StructureDefinitionKind::Resource => Self::Resource,
		}
	}
}

impl From<r4b::codes::StructureDefinitionKind> for StructureDefinitionKind {
	fn from(value: r4b::codes::StructureDefinitionKind) -> Self {
		match value {
			r4b::codes::StructureDefinitionKind::ComplexType => Self::ComplexType,
			r4b::codes::StructureDefinitionKind::Logical => Self::Logical,
			r4b::codes::StructureDefinitionKind::PrimitiveType => Self::PrimitiveType,
			r4b::codes::StructureDefinitionKind::Resource => Self::Resource,
		}
	}
}

impl From<r5::codes::StructureDefinitionKind> for StructureDefinitionKind {
	fn from(value: r5::codes::StructureDefinitionKind) -> Self {
		match value {
			r5::codes::StructureDefinitionKind::ComplexType => Self::ComplexType,
			r5::codes::StructureDefinitionKind::Logical => Self::Logical,
			r5::codes::StructureDefinitionKind::PrimitiveType => Self::PrimitiveType,
			r5::codes::StructureDefinitionKind::Resource => Self::Resource,
		}
	}
}

impl From<stu3::codes::CodeSystemContentMode> for CodeSystemContentMode {
	fn from(value: stu3::codes::CodeSystemContentMode) -> Self {
		match value {
			stu3::codes::CodeSystemContentMode::Complete => Self::Complete,
			stu3::codes::CodeSystemContentMode::Example => Self::Example,
			stu3::codes::CodeSystemContentMode::Fragment => Self::Fragment,
			stu3::codes::CodeSystemContentMode::NotPresent => Self::NotPresent,
		}
	}
}

impl From<r4b::codes::CodeSystemContentMode> for CodeSystemContentMode {
	fn from(value: r4b::codes::CodeSystemContentMode) -> Self {
		match value {
			r4b::codes::CodeSystemContentMode::Complete => Self::Complete,
			r4b::codes::CodeSystemContentMode::Example => Self::Example,
			r4b::codes::CodeSystemContentMode::Fragment => Self::Fragment,
			r4b::codes::CodeSystemContentMode::NotPresent => Self::NotPresent,
			r4b::codes::CodeSystemContentMode::Supplement => Self::Supplement,
		}
	}
}

impl From<r5::codes::CodeSystemContentMode> for CodeSystemContentMode {
	fn from(value: r5::codes::CodeSystemContentMode) -> Self {
		match value {
			r5::codes::CodeSystemContentMode::Complete => Self::Complete,
			r5::codes::CodeSystemContentMode::Example => Self::Example,
			r5::codes::CodeSystemContentMode::Fragment => Self::Fragment,
			r5::codes::CodeSystemContentMode::NotPresent => Self::NotPresent,
			r5::codes::CodeSystemContentMode::Supplement => Self::Supplement,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_types_from_structure_definitions() {
		let included_types = include_str!("../../definitions/stu3/profiles-types.json");
		let _types = structures::parse_stu3(included_types);
		let included_resources = include_str!("../../definitions/stu3/profiles-resources.json");
		let _types = structures::parse_stu3(included_resources);

		let included_types = include_str!("../../definitions/r4b/profiles-types.json");
		let _types = structures::parse_r4b(included_types);
		let included_resources = include_str!("../../definitions/r4b/profiles-resources.json");
		let _types = structures::parse_r4b(included_resources);

		let included_types = include_str!("../../definitions/r5/profiles-types.json");
		let _types = structures::parse_r5(included_types);
		let included_resources = include_str!("../../definitions/r5/profiles-resources.json");
		let _types = structures::parse_r5(included_resources);
	}

	#[test]
	fn parse_value_sets_from_code_systems() {
		let included = include_str!("../../definitions/stu3/valuesets.json");
		let _codes = codes::parse_stu3(included);
		let included = include_str!("../../definitions/r4b/valuesets.json");
		let _codes = codes::parse_r4b(included);
		let included = include_str!("../../definitions/r5/valuesets.json");
		let _codes = codes::parse_r5(included);
	}
}

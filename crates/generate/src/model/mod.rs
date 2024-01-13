//! Common model for codes and structures to generate code from.

pub mod codes;
pub mod structures;

/// Internal publication status, for all versions.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PublicationStatus {
	Active,
	Draft,
	Retired,
	Unknown,
}

/// Internal structure definition kind, for all versions.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StructureDefinitionKind {
	ComplexType,
	Logical,
	PrimitiveType,
	Resource,
}

/// Internal code system content mode, for all version.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CodeSystemContentMode {
	Complete,
	Example,
	Fragment,
	NotPresent,
	Supplement,
}

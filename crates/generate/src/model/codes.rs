//! Codes definitions.

use super::{CodeSystemContentMode, PublicationStatus};

/// Code definition.
#[derive(Debug)]
pub struct Code {
	/// Name of the code.
	pub name: String,
	/// FHIR version.
	pub version: Option<String>,
	/// Description.
	pub description: Option<String>,
	/// Status of the definition.
	pub status: PublicationStatus,
	/// Whether it is experimental.
	pub experimental: bool,
	/// Whether the codes are case sensitive.
	pub case_sensitive: bool,
	/// Whether this code is a value set (controls whether there are custom
	/// values allowed I think?).
	pub is_value_set: bool,
	/// Code ValueSet or system URL.
	pub system: String,
	/// Content mode.
	pub content: CodeSystemContentMode,
	/// Code items:
	pub items: Vec<CodeItem>,
}

/// Code item information.
#[derive(Debug)]
pub struct CodeItem {
	/// The value.
	pub code: String,
	/// The human version to be displayed.
	pub display: Option<String>,
	/// Definition of the value.
	pub definition: Option<String>,
}

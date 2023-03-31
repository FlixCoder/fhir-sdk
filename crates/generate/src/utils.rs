//! Utils.

use std::str::FromStr;

/// Status of a Definition. (TODO: Replace with FHIR RequestStatus)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
	/// Draft.
	Draft,
	/// Active.
	Active,
	/// Retired.
	Retired,
	/// Unknown.
	Unknown,
}

impl FromStr for Status {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"draft" => Ok(Self::Draft),
			"active" => Ok(Self::Active),
			"retired" => Ok(Self::Retired),
			"unknown" => Ok(Self::Unknown),
			_ => Err(format!("Unknown status: {s}")),
		}
	}
}

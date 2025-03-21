//! Error implementation.

#[cfg(feature = "builders")]
use derive_builder::UninitializedFieldError;

/// Wrong resource type for conversion to the specified type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrongResourceType(pub String, pub String);

impl std::fmt::Display for WrongResourceType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "The Resource is of a different type ({}) than requested ({})", self.0, self.1)
	}
}

impl std::error::Error for WrongResourceType {}

/// Error that may occur during the String to Date conversion
#[derive(Debug)]
pub enum DateFormatError {
	/// Date parsing error
	TimeParsing(time::error::Parse),
	/// Integer to Month conversion error
	TimeComponentRange(time::error::ComponentRange),
	/// String to Integer conversion error
	IntParsing(std::num::ParseIntError),
	/// String splitting error
	StringSplit,
	/// Incorrectly formatted date error
	InvalidDate,
}

impl std::fmt::Display for DateFormatError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::TimeParsing(err) => write!(f, "Couldn't parse date: {err}"),
			Self::TimeComponentRange(err) => write!(f, "Invalid month: {err}"),
			Self::IntParsing(err) => write!(f, "Couldn't parse string to integer: {err}"),
			Self::StringSplit => f.write_str("Couldn't split string"),
			Self::InvalidDate => f.write_str("Invalid date format"),
		}
	}
}

impl std::error::Error for DateFormatError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::TimeParsing(err) => Some(err),
			Self::TimeComponentRange(err) => Some(err),
			Self::IntParsing(err) => Some(err),
			_ => None,
		}
	}
}

impl From<time::error::Parse> for DateFormatError {
	fn from(value: time::error::Parse) -> Self {
		Self::TimeParsing(value)
	}
}

impl From<time::error::ComponentRange> for DateFormatError {
	fn from(value: time::error::ComponentRange) -> Self {
		Self::TimeComponentRange(value)
	}
}

impl From<std::num::ParseIntError> for DateFormatError {
	fn from(value: std::num::ParseIntError) -> Self {
		Self::IntParsing(value)
	}
}

#[cfg(feature = "builders")]
/// Builder errors.
#[derive(Debug)]
pub struct BuilderError(pub UninitializedFieldError);

#[cfg(feature = "builders")]
impl std::fmt::Display for BuilderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

#[cfg(feature = "builders")]
impl std::error::Error for BuilderError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		Some(&self.0)
	}
}

#[cfg(feature = "builders")]
impl From<UninitializedFieldError> for BuilderError {
	fn from(err: UninitializedFieldError) -> Self {
		Self(err)
	}
}

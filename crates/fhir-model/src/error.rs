//! Error implementation.

#[cfg(feature = "builders")]
use derive_builder::UninitializedFieldError;

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

impl std::error::Error for DateFormatError {}

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
impl std::error::Error for BuilderError {}

#[cfg(feature = "builders")]
impl From<UninitializedFieldError> for BuilderError {
	fn from(err: UninitializedFieldError) -> Self {
		Self(err)
	}
}

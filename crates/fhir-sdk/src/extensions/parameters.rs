//! Trait for additional functionality of Parameters and related types.

use std::fmt::Debug;

use fhir_model::for_all_versions;
use serde::{Serialize, de::DeserializeOwned};

/// Trait for additional functionality of Parameters. Only implemented if "builders" feature is
/// active.
pub trait ParametersExt {
	/// Inner `ParametersParameter` type.
	type Parameter: ParameterExt
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;

	/// Make a `Parameters` instance using the inner parameters.
	fn make(parameters: Vec<Option<Self::Parameter>>) -> Self;
}

/// Implement `ParametersExt` for all `Parameters` versions.
macro_rules! impl_parameters_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::{Parameters, ParametersParameter};

			use super::*;

			impl ParametersExt for Parameters {
				type Parameter = ParametersParameter;

				#[inline]
				fn make(parameters: Vec<Option<Self::Parameter>>) -> Self {
					#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
					Self::builder().parameter(parameters).build().unwrap()
				}
			}
		}
	};
}
#[cfg(feature = "builders")]
mod helper_module_1 {
	//! Helper module to avoid conflicts.
	use super::*;
	for_all_versions!(impl_parameters_ext);
}

/// Trait for additional functionality of `ParametersParameter`. Only implemented if "builders"
/// feature is active.
pub trait ParameterExt: Sized {
	/// Inner `ParametersParameterValue` type.
	type Value: ParameterValueExt
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;

	/// Make a `ParametersParameter` instance using the inner parameters.
	fn make(name: String, value: Option<Self::Value>, part: Vec<Option<Self>>) -> Self;
}

/// Implement `ParameterExt` for all `ParametersParameter` versions.
macro_rules! impl_parameters_parameter_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::{ParametersParameter, ParametersParameterValue};

			use super::*;

			impl ParameterExt for ParametersParameter {
				type Value = ParametersParameterValue;

				#[inline]
				fn make(name: String, value: Option<Self::Value>, part: Vec<Option<Self>>) -> Self {
					#[allow(clippy::unwrap_used, reason = "We know the builder succeeds")]
					if let Some(value) = value {
						Self::builder().name(name).value(value).part(part).build().unwrap()
					} else {
						Self::builder().name(name).part(part).build().unwrap()
					}
				}
			}
		}
	};
}
#[cfg(feature = "builders")]
mod helper_module_2 {
	//! Helper module to avoid conflicts.
	use super::*;
	for_all_versions!(impl_parameters_parameter_ext);
}

/// Trait for additional functionality of `ParametersParameterValue`.
pub trait ParameterValueExt {
	/// Make a `ParametersParameterValue::Code` instance using the inner
	/// parameters.
	fn make_code(code: String) -> Self;
	/// Make a `ParametersParameterValue::String` instance using the inner
	/// parameters.
	fn make_string(value: String) -> Self;
	/// Make a `ParametersParameterValue::Integer` instance using the inner
	/// parameters.
	fn make_integer(value: i32) -> Self;
}

/// Implement `ParameterValueExt` for all `ParametersParameterValue` versions.
macro_rules! impl_parameter_value_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::ParametersParameterValue;

			use super::*;

			impl ParameterValueExt for ParametersParameterValue {
				#[inline]
				fn make_code(code: String) -> Self {
					Self::Code(code)
				}

				#[inline]
				fn make_string(value: String) -> Self {
					Self::String(value)
				}

				#[inline]
				fn make_integer(value: i32) -> Self {
					Self::Integer(value)
				}
			}
		}
	};
}
mod helper_module_3 {
	//! Helper module to avoid conflicts.
	use super::*;
	for_all_versions!(impl_parameter_value_ext);
}

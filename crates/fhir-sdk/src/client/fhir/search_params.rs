//! Client search implementation.

use std::marker::PhantomData;

use super::super::{misc::escape_search_value, search::SearchParameter};
use crate::version::FhirVersion;

/// Number search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone)]
pub struct NumberSearch<'a, V> {
	/// Name of the field.
	name: &'a str,
	/// Values encoded as string already (will be comma-separated for
	/// OR-joining).
	values: Vec<String>,
	/// Use V.
	_version: PhantomData<V>,
}

impl<'a, V: FhirVersion> NumberSearch<'a, V> {
	/// Start with empty values and add values one at a time.
	#[must_use]
	pub const fn new(name: &'a str) -> Self {
		Self { name, values: Vec::new(), _version: PhantomData }
	}

	/// Add a value to the number search.
	#[must_use]
	pub fn or<T: ToString>(mut self, comparator: Option<V::SearchComparator>, value: T) -> Self {
		let value = escape_search_value(&value.to_string());
		if let Some(comparator) = comparator {
			self.values.push(format!("{comparator}{value}"));
		} else {
			self.values.push(value);
		}
		self
	}
}

impl<V> SearchParameter for NumberSearch<'_, V> {
	fn into_query(self) -> (String, String) {
		(self.name.to_owned(), self.values.join(","))
	}
}

/// Date search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub struct DateSearch<'a, V: FhirVersion> {
	/// Name of the field.
	pub name: &'a str,
	/// Search comparator to compare the date.
	pub comparator: Option<V::SearchComparator>,
	/// Value to search for.
	pub value: &'a str,
}

impl<V: FhirVersion> SearchParameter for DateSearch<'_, V> {
	fn into_query(self) -> (String, String) {
		if let Some(comparator) = self.comparator {
			(
				self.name.to_owned(),
				format!("{}{}", comparator.as_ref(), escape_search_value(self.value)),
			)
		} else {
			(self.name.to_owned(), escape_search_value(self.value))
		}
	}
}

/// String search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum StringSearch<'a> {
	/// Standard string search. This is a case-insensitive starts-with search.
	Standard {
		/// Name of the field to search on.
		name: &'a str,
		/// Value of the field.
		value: &'a str,
	},
	/// Search a string that contains the given value.
	Contains {
		/// Name of the field to search on.
		name: &'a str,
		/// Value of the field.
		value: &'a str,
	},
	/// Search a string that matches exactly the value, including casing and
	/// accents.
	Exact {
		/// Name of the field to search on.
		name: &'a str,
		/// Value of the field.
		value: &'a str,
	},
}

impl SearchParameter for StringSearch<'_> {
	fn into_query(self) -> (String, String) {
		let (name, modifier, value) = match self {
			Self::Standard { name, value } => (name, "", value),
			Self::Contains { name, value } => (name, ":contains", value),
			Self::Exact { name, value } => (name, ":exact", value),
		};
		(format!("{name}{modifier}"), escape_search_value(value))
	}
}

/// Token search, e.g. in `CodeableConcept`s or `identifier`s.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum TokenSearch<'a> {
	/// Standard token search (or `not` search).
	Standard {
		/// Name of the field to search on.
		name: &'a str,
		/// System for the value to search on. If this is given as empty string,
		/// the system must not be present.
		system: Option<&'a str>,
		/// Value of the code to search on.
		code: Option<&'a str>,
		/// Whether to switch to the `not` modifier.
		not: bool,
	},
	/// Token search with the `of-type` modifier. Only possible on
	/// `identifier`s.
	OfType {
		/// System of type to search on.
		type_system: Option<&'a str>,
		/// Code of the type to search on.
		type_code: Option<&'a str>,
		/// Value to search on with the given type.
		value: Option<&'a str>,
	},
	/// Token search whether the value is `in` or `not-in` a given `ValueSet`.
	In {
		/// Name of the field to search on.
		name: &'a str,
		/// `ValueSet` reference URI.
		value_set: &'a str,
		/// Whether to switch to `not-in` search.
		not: bool,
	},
	/// Tests the `text` or `display` values.
	CodeText {
		/// Name of the field to search on.
		name: &'a str,
		/// Text to search for (is a starts-with search).
		text: &'a str,
	},
}

impl SearchParameter for TokenSearch<'_> {
	fn into_query(self) -> (String, String) {
		match self {
			Self::Standard { name, system, code, not } => {
				let key = if not { format!("{name}:not") } else { name.to_owned() };
				let value = if let Some(system) = system {
					format!(
						"{}|{}",
						escape_search_value(system),
						escape_search_value(code.unwrap_or_default())
					)
				} else {
					escape_search_value(code.unwrap_or_default())
				};
				(key, value)
			}
			Self::OfType { type_system, type_code, value } => (
				"identifier:of-type".to_owned(),
				format!(
					"{}|{}|{}",
					escape_search_value(type_system.unwrap_or_default()),
					escape_search_value(type_code.unwrap_or_default()),
					escape_search_value(value.unwrap_or_default())
				),
			),
			Self::In { name, value_set, not } => {
				if not {
					(format!("{name}:not-in"), escape_search_value(value_set))
				} else {
					(format!("{name}:in"), escape_search_value(value_set))
				}
			}
			Self::CodeText { name, text } => {
				(format!("{name}:code-text"), escape_search_value(text))
			}
		}
	}
}

/// Search in references. Includes chaining, i.e. querying fields of the target
/// resource.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum ReferenceSearch<'a, V: FhirVersion> {
	/// Standard reference search by relative reference.
	Standard {
		/// Name of the field.
		name: &'a str,
		/// Resource type of the resource.
		resource_type: V::ResourceType,
		/// ID of the resource the reference should point to.
		id: &'a str,
		/// Historic version id to search for.
		version_id: Option<&'a str>,
	},
	/// Standard reference search by absolute URL.
	Url {
		/// Name of the field.
		name: &'a str,
		/// Reference URL.
		url: &'a str,
		/// Specific version id to search for.
		version_id: Option<&'a str>,
	},
	/// Reference search by the `.identifier` field.
	Identifier {
		/// Name of the field.
		name: &'a str,
		/// Identifier system.
		system: Option<&'a str>,
		/// Idenfifier value.
		value: Option<&'a str>,
	},
	/// Reference search with chaining. This means the server searches for
	/// references that target a resource with the given field and value.
	Chaining {
		/// Name of the field.
		name: &'a str,
		/// Resource type of the reference.
		resource_type: Option<V::ResourceType>,
		/// Target resource field name.
		target_name: &'a str,
		/// (Raw) value of the target value. Might be any of the ways of search,
		/// so could be token search including pipes.
		value: &'a str,
	},
}

impl<V: FhirVersion> SearchParameter for ReferenceSearch<'_, V> {
	fn into_query(self) -> (String, String) {
		match self {
			Self::Standard { name, resource_type, id, version_id } => {
				let value = if let Some(version_id) = version_id {
					escape_search_value(&format!("{resource_type}/{id}/_history/{version_id}"))
				} else {
					escape_search_value(&format!("{resource_type}/{id}"))
				};
				(name.to_owned(), value)
			}
			Self::Url { name, url, version_id } => {
				let value = if let Some(version_id) = version_id {
					format!("{}|{}", escape_search_value(url), escape_search_value(version_id))
				} else {
					escape_search_value(url)
				};
				(name.to_owned(), value)
			}
			Self::Identifier { name, system, value } => (
				name.to_owned(),
				format!(
					"{}|{}",
					escape_search_value(system.unwrap_or_default()),
					escape_search_value(value.unwrap_or_default()),
				),
			),
			Self::Chaining { name, resource_type, target_name, value } => {
				let key = if let Some(resource_type) = resource_type {
					format!("{name}:{resource_type}.{target_name}")
				} else {
					format!("{name}.{target_name}")
				};
				(key, value.to_owned())
			}
		}
	}
}

/// Search on a quantity.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub struct QuantitySearch<'a, V: FhirVersion> {
	/// Name of the field.
	pub name: &'a str,
	/// Search comparator to compare the date.
	pub comparator: Option<V::SearchComparator>,
	/// Value to search for.
	pub value: &'a str,
	/// Optional system.
	pub system: Option<&'a str>,
	/// Optional code.
	pub code: Option<&'a str>,
}

impl<V: FhirVersion> SearchParameter for QuantitySearch<'_, V> {
	fn into_query(self) -> (String, String) {
		let value = if let Some(comparator) = self.comparator {
			format!("{comparator}{}", escape_search_value(self.value))
		} else {
			escape_search_value(self.value)
		};

		let query_value = if self.system.is_some() || self.code.is_some() {
			format!(
				"{value}|{}|{}",
				escape_search_value(self.system.unwrap_or_default()),
				escape_search_value(self.code.unwrap_or_default())
			)
		} else {
			value
		};

		(self.name.to_owned(), query_value)
	}
}

/// Search on a URI.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum UriSearch<'a> {
	/// Standard URI search, that matches exactly.
	Standard {
		/// Name of the field.
		name: &'a str,
		/// URI.
		uri: &'a str,
	},
	/// Match any URL that is below the given URL path, so contains more URL
	/// segments.
	Below {
		/// Name of the field.
		name: &'a str,
		/// URI.
		uri: &'a str,
	},
	/// Match any URL that is above the given URL path, so contains less URL
	/// segments.
	Above {
		/// Name of the field.
		name: &'a str,
		/// URI.
		uri: &'a str,
	},
}

impl SearchParameter for UriSearch<'_> {
	fn into_query(self) -> (String, String) {
		let (name, modifier, uri) = match self {
			Self::Standard { name, uri } => (name, "", uri),
			Self::Below { name, uri } => (name, ":below", uri),
			Self::Above { name, uri } => (name, ":above", uri),
		};
		(format!("{name}{modifier}"), escape_search_value(uri))
	}
}

/// Search on any item whether it is a missing field using the `missing`
/// modifier.
#[derive(Debug, Clone, Copy)]
pub struct MissingSearch<'a> {
	/// Name of the field.
	pub name: &'a str,
	/// Whether to search for the absent field (or the present).
	pub missing: bool,
}

impl SearchParameter for MissingSearch<'_> {
	fn into_query(self) -> (String, String) {
		(format!("{}:missing", self.name), self.missing.to_string())
	}
}

#[cfg(test)]
mod tests {
	use fhir_model::for_all_versions;

	use super::*;
	use crate::version::fhir_version;

	macro_rules! make_tests {
		($version:ident) => {
			mod $version {
				use $crate::$version::{codes::SearchComparator, resources::ResourceType};

				use super::*;

				#[test]
				fn number() {
					let number = NumberSearch::<fhir_version!($version)>::new("value-quantity")
						.or(Some(SearchComparator::Lt), 60)
						.or(Some(SearchComparator::Gt), 100);
					assert_eq!(
						number.into_query(),
						("value-quantity".to_owned(), "lt60,gt100".to_owned())
					);
				}

				#[test]
				fn token() {
					let token = TokenSearch::Standard {
						name: "identifier",
						system: None,
						code: Some("code"),
						not: true,
					};
					assert_eq!(
						token.into_query(),
						("identifier:not".to_owned(), "code".to_owned())
					);

					let token = TokenSearch::Standard {
						name: "identifier",
						system: Some(""),
						code: Some("code"),
						not: false,
					};
					assert_eq!(token.into_query(), ("identifier".to_owned(), "|code".to_owned()));

					let token = TokenSearch::Standard {
						name: "identifier",
						system: Some("system"),
						code: None,
						not: false,
					};
					assert_eq!(token.into_query(), ("identifier".to_owned(), "system|".to_owned()));

					let token = TokenSearch::OfType {
						type_system: None,
						type_code: None,
						value: Some("value"),
					};
					assert_eq!(
						token.into_query(),
						("identifier:of-type".to_owned(), "||value".to_owned())
					);
				}

				#[test]
				fn reference() {
					let reference: ReferenceSearch<'_, fhir_version!($version)> =
						ReferenceSearch::Chaining {
							name: "focus",
							resource_type: Some(ResourceType::Encounter),
							target_name: "status",
							value: "in-progress",
						};
					assert_eq!(
						reference.into_query(),
						("focus:Encounter.status".to_owned(), "in-progress".to_owned())
					);
				}

				#[test]
				fn quantity() {
					let quantity = QuantitySearch::<fhir_version!($version)> {
						name: "test",
						comparator: None,
						value: "1.0",
						system: None,
						code: None,
					};
					assert_eq!(quantity.into_query(), ("test".to_owned(), "1.0".to_owned()));

					let quantity = QuantitySearch::<fhir_version!($version)> {
						name: "test",
						comparator: None,
						value: "1.0",
						system: None,
						code: Some("g"),
					};
					assert_eq!(quantity.into_query(), ("test".to_owned(), "1.0||g".to_owned()));
				}

				#[test]
				fn missing() {
					let missing = MissingSearch { name: "identifier", missing: true };
					assert_eq!(
						missing.into_query(),
						("identifier:missing".to_owned(), "true".to_owned())
					);
				}
			}
		};
	}
	for_all_versions!(make_tests);
}

//! Search parameter handling.

/// A collection of AND-joined search parameters.
#[derive(Debug, Default, Clone)]
pub struct SearchParameters {
	/// List of search queries.
	queries: Vec<(String, String)>,
}

impl SearchParameters {
	/// Create a new list of [`SearchParameters`].
	#[must_use]
	pub fn empty() -> Self {
		Self::default()
	}

	/// Add a search parameter.
	#[must_use]
	pub fn and<P>(mut self, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.queries.push(parameter.into_query());
		self
	}

	/// Add a raw custom search parameter.
	///
	/// The key of the search query includes modifiers or chaining.
	///
	/// The value of the search query might include multiple comma-separated
	/// values. A value can consist of pipe-separated values for token search or
	/// can be prepended by a comparator like `lt` for numbers.
	#[must_use]
	pub fn and_raw(mut self, key: impl Into<String>, value: impl ToString) -> Self {
		self.queries.push((key.into(), value.to_string()));
		self
	}

	/// Convert to a list of raw queries.
	pub(crate) fn into_queries(self) -> Vec<(String, String)> {
		self.queries
	}
}

/// Functionality to convert a SearchParameter to the URL query.
pub trait SearchParameter {
	/// Convert this search parameter into the query pair (key, value).
	fn into_query(self) -> (String, String);
}

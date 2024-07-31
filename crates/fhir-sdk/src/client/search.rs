//! Search parameter handling.

use crate::Url;

/// Method to use for paged search.
/// Use Parameters for the initial search, and Url to "follow" previous and next pages.
#[derive(Debug, Clone)]
pub enum PagedSearchMethod {
	/// Search by given parameters, i.e., make the initial request.
	Parameters(SearchParameters),
	/// Search by given exact URL, i.e., make follow-up request.
	Url(Url),
}

/// Single page of a result.
///
/// The URLs to previous and next pages are opaque and can be anything.
/// The client only ensures it access to the same server.
///
/// See [search] for details and examples.
#[derive(Debug, Clone)]
pub struct Page<R> {
	/// Results matching the search.
	pub results: Vec<R>,
	/// The URL to the previous page.
	pub previous_url: Option<Url>,
	/// The URL to the next page.
	pub next_url: Option<Url>,
	/// Total number of matches, as defined by the server.
	/// This field is always optional and  behavior for this can be controlled with _total search parameter,
	/// but the server can ignore it and
	pub total: Option<u32>,
}

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

/// Escape a search parameter value.
pub(crate) fn escape_value(value: &str) -> String {
	value.replace('\\', "\\\\").replace('|', "\\|").replace('$', "\\$").replace(',', "\\,")
}

//! FHIR API access methods (version specific).

mod crud;
mod paging;
mod patch;
mod search_params;
mod transaction;
mod write;

pub use self::{
	search_params::{
		DateSearch, MissingSearch, NumberSearch, QuantitySearch, ReferenceSearch, StringSearch,
		TokenSearch, UriSearch,
	},
	write::{AnyResourceWrite, ResourceWrite},
};
use super::{misc, Client, Error, SearchParameters};

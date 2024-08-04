//! FHIR API access methods (version specific).

mod crud;
mod search_params;
mod write;

pub use self::{
	search_params::{
		DateSearch, MissingSearch, NumberSearch, QuantitySearch, ReferenceSearch, StringSearch,
		TokenSearch, UriSearch,
	},
	write::{AnyResourceWrite, ResourceWrite},
};

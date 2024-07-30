#![cfg_attr(
	all(feature = "r5", feature = "client", feature = "builders", feature = "docs"),
	doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
)]
#![cfg_attr(
	not(all(feature = "r5", feature = "client", feature = "builders", feature = "docs")),
	doc = "Enable the following features to see the crate-level documentation: r5, client, docs"
)]

/// Run a macro for all FHIR versions to implement similar things for different
/// FHIR versions.
macro_rules! for_all_versions {
	($macro:ident) => {
		$macro!(stu3);
		$macro!(r4b);
		$macro!(r5);
	};
}

#[cfg(feature = "client")]
pub mod client;
pub mod extensions;

pub use fhir_model::*;
#[cfg(feature = "client")]
pub use futures::stream::{Stream, StreamExt, TryStream, TryStreamExt};
#[cfg(feature = "client")]
pub use reqwest::{
	header::{self, HeaderMap, HeaderValue},
	Body, Client as HttpClient, Request, RequestBuilder, Response, StatusCode, Url,
};

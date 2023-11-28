#![cfg_attr(
	all(feature = "r5", feature = "builders", feature = "client", feature = "docs"),
	doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
)]
#![cfg_attr(
	not(all(feature = "r5", feature = "builders", feature = "client", feature = "docs")),
	doc = "Enable the following features to see the crate-level documentation: r5, builders, client, docs"
)]

#[cfg(feature = "client")]
pub mod client;

pub use fhir_model::*;
#[cfg(feature = "client")]
pub use futures::stream::{Stream, StreamExt, TryStream, TryStreamExt};
#[cfg(feature = "client")]
pub use reqwest::Url;

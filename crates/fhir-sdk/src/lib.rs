#![cfg_attr(
	all(feature = "r5", feature = "builders", feature = "client", feature = "docs"),
	doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))
)]

#[cfg(feature = "client")]
pub mod client;

pub use fhir_model::*;

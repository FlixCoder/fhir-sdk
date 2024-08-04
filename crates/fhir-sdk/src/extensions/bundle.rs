//! Generalized functionality of `Bundle`s.

use std::{
	fmt::{Debug, Display},
	str::FromStr,
};

use fhir_model::for_all_versions;
use serde::{de::DeserializeOwned, Serialize};

use super::{GenericResource, SearchEntryModeExt};

/// Additional/generalized functionality on `Bundle`s.
pub trait BundleExt {
	/// Entry type.
	type Entry: BundleEntryExt
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;

	/// See [Bundle::next_page_url].
	fn next_page_url(&self) -> Option<&String>;
	/// Iterate over entries.
	fn entries(&self) -> impl Iterator<Item = &Self::Entry>;
	/// Iterate over owned entries, consuming this Bundle.
	fn into_entries(self) -> impl Iterator<Item = Self::Entry>;
}

macro_rules! impl_bundle_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::resources::{Bundle, BundleEntry};

			use super::*;

			impl BundleExt for Bundle {
				type Entry = BundleEntry;

				fn next_page_url(&self) -> Option<&String> {
					Bundle::next_page_url(self)
				}

				fn entries(&self) -> impl Iterator<Item = &Self::Entry> {
					self.0.entry.iter().flatten()
				}

				fn into_entries(self) -> impl Iterator<Item = Self::Entry> {
					self.0.entry.into_iter().flatten()
				}
			}
		}
	};
}
mod bundle_ext {
	use super::*;
	for_all_versions!(impl_bundle_ext);
}

/// Additional/generalized functionality on `BundleEntry`s.
pub trait BundleEntryExt {
	/// Generic resource enum for this version.
	type Resource: GenericResource
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ Clone
		+ PartialEq
		+ Unpin
		+ Send
		+ Sync;
	/// Search entry mode.
	type SearchEntryMode: SearchEntryModeExt
		+ Serialize
		+ DeserializeOwned
		+ Debug
		+ FromStr
		+ AsRef<str>
		+ Display
		+ Clone
		+ Copy
		+ PartialEq
		+ Eq
		+ Unpin
		+ Send
		+ Sync;

	/// Get the search.mode field.
	fn search_mode(&self) -> Option<&Self::SearchEntryMode>;
	/// Get the full URL field.
	fn full_url(&self) -> Option<&String>;
	/// Get the inner resource.
	fn resource(&self) -> Option<&Self::Resource>;
	/// Consume the entry and turn it into its relevant parts.
	fn into_parts(self) -> (Option<String>, Option<Self::Resource>);
}

macro_rules! impl_bundle_entry_ext {
	($version:ident) => {
		mod $version {
			use fhir_model::$version::{
				codes::SearchEntryMode,
				resources::{BundleEntry, Resource},
			};

			use super::*;

			impl BundleEntryExt for BundleEntry {
				type Resource = Resource;
				type SearchEntryMode = SearchEntryMode;

				fn search_mode(&self) -> Option<&Self::SearchEntryMode> {
					self.search.as_ref().and_then(|search| search.mode.as_ref())
				}

				fn full_url(&self) -> Option<&String> {
					self.full_url.as_ref()
				}

				fn resource(&self) -> Option<&Self::Resource> {
					self.resource.as_ref()
				}

				fn into_parts(self) -> (Option<String>, Option<Self::Resource>) {
					(self.full_url, self.resource)
				}
			}
		}
	};
}
mod bundle_entry_ext {
	use super::*;
	for_all_versions!(impl_bundle_entry_ext);
}

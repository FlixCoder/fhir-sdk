//! Extension traits on resources.
#![cfg_attr(not(feature = "builders"), allow(dead_code, unused_imports, unused_macros,))] // Many impls missing without builders.

mod any_resource;
mod bundle;
mod codes;
mod generic_resource;
mod parameters;
mod references;

pub use self::any_resource::AnyResource;
#[allow(unused_imports, reason = "Feature specific")]
pub(crate) use self::{bundle::*, codes::*, generic_resource::*, parameters::*, references::*};

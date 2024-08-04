//! Extension traits on resources.

mod any_resource;
mod bundle;
mod codes;
mod generic_resource;
mod references;

pub use self::any_resource::AnyResource;
pub(crate) use self::{bundle::*, codes::*, generic_resource::*, references::*};

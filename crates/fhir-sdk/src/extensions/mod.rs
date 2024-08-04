//! Extension traits on resources.

mod any_resource;
mod generic_resource;
mod references;

pub use self::any_resource::AnyResource;
pub(crate) use self::{generic_resource::*, references::*};

//! Utils for internal use (outside client).

/// Crate internal trait to disallow users to implement some trait externally.
/// Do not publicly export :D
pub trait Sealed {}

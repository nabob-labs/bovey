//! Bovey precompiled programs.

#![cfg(feature = "full")]

#[deprecated(since = "2.1.0", note = "Use `bovey-precompile-error` crate instead.")]
pub use bovey_precompile_error::PrecompileError;
#[deprecated(since = "2.2.0", note = "Use `bovey-precompiles` crate instead.")]
pub use bovey_precompiles::{
    get_precompile, get_precompiles, is_precompile, verify_if_precompile, Precompile, Verify,
};

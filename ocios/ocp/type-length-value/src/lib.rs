//! Crate defining an interface for managing type-length-value entries in a slab
//! of bytes, to be used with Bovey accounts.

#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod length;
pub mod state;
pub mod variable_len_pack;

// Export current sdk types for downstream users building with a different sdk
// version
// Expose derive macro on feature flag
#[cfg(feature = "derive")]
pub use ocp_type_length_value_derive::OcpBorshVariableLenPack;
pub use {bovey_account_info, bovey_decode_error, bovey_program_error};

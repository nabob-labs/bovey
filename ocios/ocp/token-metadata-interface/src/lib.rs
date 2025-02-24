//! Crate defining an interface for token-metadata

#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod instruction;
pub mod state;

// Export current sdk types for downstream users building with a different sdk
// version Export borsh for downstream users
pub use {
    borsh, bovey_borsh, bovey_decode_error, bovey_instruction, bovey_msg, bovey_program_error,
};

/// Namespace for all programs implementing token-metadata
pub const NAMESPACE: &str = "ocp_token_metadata_interface";

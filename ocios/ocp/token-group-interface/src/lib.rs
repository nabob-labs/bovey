//! Crate defining the OCP Token Group Interface

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod instruction;
pub mod state;

/// Namespace for all programs implementing ocp-token-group
pub const NAMESPACE: &str = "ocp_token_group_interface";

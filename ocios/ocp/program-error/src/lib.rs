//! Crate defining a library with a procedural macro and other
//! dependencies for building Bovey program errors

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate self as ocp_program_error;

// Make these available downstream for the macro to work without
// additional imports
pub use {
    num_derive, num_traits, bovey_program,
    ocp_program_error_derive::{
        ocp_program_error, DecodeError, IntoProgramError, PrintProgramError,
    },
    thiserror,
};

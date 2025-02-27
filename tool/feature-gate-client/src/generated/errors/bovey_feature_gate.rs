//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>

use {num_derive::FromPrimitive, thiserror::Error};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum BoveyFeatureGateError {
    /// 0 - Feature already activated
    #[error("Feature already activated")]
    FeatureAlreadyActivated = 0x0,
}

impl bovey_program::program_error::PrintProgramError for BoveyFeatureGateError {
    fn print<E>(&self) {
        bovey_program::msg!(&self.to_string());
    }
}

impl<T> bovey_program::decode_error::DecodeError<T> for BoveyFeatureGateError {
    fn type_of() -> &'static str {
        "BoveyFeatureGateError"
    }
}

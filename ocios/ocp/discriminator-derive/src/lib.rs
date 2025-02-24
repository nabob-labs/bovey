//! Derive macro library for the `ocp-discriminator` library

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate proc_macro;

use {
    proc_macro::TokenStream, quote::ToTokens, ocp_discriminator_syn::OcpDiscriminateBuilder,
    syn::parse_macro_input,
};

/// Derive macro library to implement the `OcpDiscriminate` trait
/// on an enum or struct
#[proc_macro_derive(OcpDiscriminate, attributes(discriminator_hash_input))]
pub fn ocp_discriminator(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as OcpDiscriminateBuilder)
        .to_token_stream()
        .into()
}

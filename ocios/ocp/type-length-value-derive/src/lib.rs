//! Crate defining a derive macro for a basic borsh implementation of
//! the trait `VariableLenPack`.

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate proc_macro;

mod builder;

use {
    builder::OcpBorshVariableLenPackBuilder, proc_macro::TokenStream, quote::ToTokens,
    syn::parse_macro_input,
};

/// Derive macro to add `VariableLenPack` trait for borsh-implemented types
#[proc_macro_derive(OcpBorshVariableLenPack)]
pub fn ocp_borsh_variable_len_pack(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as OcpBorshVariableLenPackBuilder)
        .to_token_stream()
        .into()
}

//! Token parsing

use {
    proc_macro2::Ident,
    syn::{
        parse::{Parse, ParseStream},
        token::Comma,
        LitInt, Token,
    },
};

/// Possible arguments to the `#[ocp_program_error]` attribute
pub struct OcpProgramErrorArgs {
    /// Whether to hash the error codes using `bovey_program::hash`
    /// or to use the default error code assigned by `num_traits`.
    pub hash_error_code_start: Option<u32>,
}

impl Parse for OcpProgramErrorArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                hash_error_code_start: None,
            });
        }
        match OcpProgramErrorArgParser::parse(input)? {
            OcpProgramErrorArgParser::HashErrorCodes { value, .. } => Ok(Self {
                hash_error_code_start: Some(value.base10_parse::<u32>()?),
            }),
        }
    }
}

/// Parser for args to the `#[ocp_program_error]` attribute
/// ie. `#[ocp_program_error(hash_error_code_start = 1275525928)]`
enum OcpProgramErrorArgParser {
    HashErrorCodes {
        _ident: Ident,
        _equals_sign: Token![=],
        value: LitInt,
        _comma: Option<Comma>,
    },
}

impl Parse for OcpProgramErrorArgParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ident = {
            let ident = input.parse::<Ident>()?;
            if ident != "hash_error_code_start" {
                return Err(input.error("Expected argument 'hash_error_code_start'"));
            }
            ident
        };
        let _equals_sign = input.parse::<Token![=]>()?;
        let value = input.parse::<LitInt>()?;
        let _comma: Option<Comma> = input.parse().unwrap_or(None);
        Ok(Self::HashErrorCodes {
            _ident,
            _equals_sign,
            value,
            _comma,
        })
    }
}

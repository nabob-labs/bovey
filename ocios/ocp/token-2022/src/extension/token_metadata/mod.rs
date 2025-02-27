use {
    crate::extension::{Extension, ExtensionType},
    ocp_token_metadata_interface::state::TokenMetadata,
};

/// Instruction processor for the `TokenMetadata` extension
pub mod processor;

impl Extension for TokenMetadata {
    const TYPE: ExtensionType = ExtensionType::TokenMetadata;
}

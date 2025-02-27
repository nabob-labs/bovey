use {
    crate::extension::{Extension, ExtensionType},
    ocp_token_group_interface::state::{TokenGroup, TokenGroupMember},
};

/// Instruction processor for the `TokenGroup` extension
pub mod processor;

impl Extension for TokenGroup {
    const TYPE: ExtensionType = ExtensionType::TokenGroup;
}

impl Extension for TokenGroupMember {
    const TYPE: ExtensionType = ExtensionType::TokenGroupMember;
}

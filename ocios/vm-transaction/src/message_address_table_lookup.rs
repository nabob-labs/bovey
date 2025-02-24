use {bovey_message::v0, bovey_pubkey::Pubkey};

/// A non-owning version of [`v0::MessageAddressTableLookup`].
/// This simply references the data in the original message.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VMMessageAddressTableLookup<'a> {
    /// Address lookup table account key
    pub account_key: &'a Pubkey,
    /// List of indexes used to load writable account addresses
    pub writable_indexes: &'a [u8],
    /// List of indexes used to load readonly account addresses
    pub readonly_indexes: &'a [u8],
}

impl<'a> From<&'a v0::MessageAddressTableLookup> for VMMessageAddressTableLookup<'a> {
    fn from(lookup: &'a v0::MessageAddressTableLookup) -> Self {
        Self {
            account_key: &lookup.account_key,
            writable_indexes: &lookup.writable_indexes,
            readonly_indexes: &lookup.readonly_indexes,
        }
    }
}

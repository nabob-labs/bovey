use {
    crate::transaction_meta::StaticMeta,
    bovey_vm_transaction::vm_transaction::VMTransaction,
    bovey_transaction::{sanitized::SanitizedTransaction, versioned::VersionedTransaction},
    std::borrow::Cow,
};

pub trait TransactionWithMeta: StaticMeta + VMTransaction {
    /// Required to interact with geyser plugins.
    /// This function should not be used except for interacting with geyser.
    /// It may do numerous allocations that negatively impact performance.
    fn as_sanitized_transaction(&self) -> Cow<SanitizedTransaction>;
    /// Required to interact with several legacy interfaces that require
    /// `VersionedTransaction`. This should not be used unless necessary, as it
    /// performs numerous allocations that negatively impact performance.
    fn to_versioned_transaction(&self) -> VersionedTransaction;
}

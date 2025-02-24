use {
    crate::vm_transaction::VMTransaction, bovey_signature::Signature,
    bovey_transaction::sanitized::SanitizedTransaction,
};

impl VMTransaction for SanitizedTransaction {
    fn signature(&self) -> &Signature {
        SanitizedTransaction::signature(self)
    }

    fn signatures(&self) -> &[Signature] {
        SanitizedTransaction::signatures(self)
    }
}

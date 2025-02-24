use {crate::vm_message::VMMessage, bovey_signature::Signature};

mod sanitized_transaction;

pub trait VMTransaction: VMMessage {
    /// Get the first signature of the message.
    fn signature(&self) -> &Signature;

    /// Get all the signatures of the message.
    fn signatures(&self) -> &[Signature];
}

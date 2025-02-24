//! Functionality for public and private keys.
#![cfg(feature = "full")]

// legacy module paths
#[deprecated(
    since = "2.2.0",
    note = "Use bovey_keypair::signable::Signable instead."
)]
pub use bovey_keypair::signable::Signable;
pub use {
    crate::signer::{keypair::*, null_signer::*, presigner::*, *},
    bovey_signature::{ParseSignatureError, Signature, SIGNATURE_BYTES},
};

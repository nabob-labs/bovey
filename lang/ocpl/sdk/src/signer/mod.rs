#![cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `bovey-presigner` crate instead")]
pub use bovey_presigner as presigner;
#[deprecated(since = "2.2.0", note = "Use `bovey-seed-derivable` crate instead")]
pub use bovey_seed_derivable::SeedDerivable;
#[deprecated(since = "2.2.0", note = "Use `bovey-signer` crate instead")]
pub use bovey_signer::{
    null_signer, signers, unique_signers, EncodableKey, EncodableKeypair, Signer, SignerError,
};
pub mod keypair;

//! Hashing with the [SHA-256] hash function, and a general [`Hash`] type.
//!
//! [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
//! [`Hash`]: struct@Hash

pub use {
    bovey_hash::{Hash, ParseHashError, HASH_BYTES},
    bovey_sha256_hasher::{extend_and_hash, hash, hashv, Hasher},
};

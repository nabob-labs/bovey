#![no_std]
#[cfg(any(feature = "sha2", not(target_os = "bovey")))]
use sha2::{Digest, Sha256};
use bovey_hash::Hash;

#[cfg(any(feature = "sha2", not(target_os = "bovey")))]
#[derive(Clone, Default)]
pub struct Hasher {
    hasher: Sha256,
}

#[cfg(any(feature = "sha2", not(target_os = "bovey")))]
impl Hasher {
    pub fn hash(&mut self, val: &[u8]) {
        self.hasher.update(val);
    }
    pub fn hashv(&mut self, vals: &[&[u8]]) {
        for val in vals {
            self.hash(val);
        }
    }
    pub fn result(self) -> Hash {
        let bytes: [u8; bovey_hash::HASH_BYTES] = self.hasher.finalize().into();
        bytes.into()
    }
}

#[cfg(target_os = "bovey")]
pub use bovey_define_syscall::definitions::bov_sha256;

/// Return a Sha256 hash for the given data.
pub fn hashv(vals: &[&[u8]]) -> Hash {
    // Perform the calculation inline, calling this from within a program is
    // not supported
    #[cfg(not(target_os = "bovey"))]
    {
        let mut hasher = Hasher::default();
        hasher.hashv(vals);
        hasher.result()
    }
    // Call via a system call to perform the calculation
    #[cfg(target_os = "bovey")]
    {
        let mut hash_result = [0; bovey_hash::HASH_BYTES];
        unsafe {
            bov_sha256(
                vals as *const _ as *const u8,
                vals.len() as u64,
                &mut hash_result as *mut _ as *mut u8,
            );
        }
        Hash::new_from_array(hash_result)
    }
}

/// Return a Sha256 hash for the given data.
pub fn hash(val: &[u8]) -> Hash {
    hashv(&[val])
}

/// Return the hash of the given hash extended with the given value.
pub fn extend_and_hash(id: &Hash, val: &[u8]) -> Hash {
    let mut hash_data = id.as_ref().to_vec();
    hash_data.extend_from_slice(val);
    hash(&hash_data)
}

//! Crate containing `Pod` types and `bytemuck` utils used in OCP

pub mod bytemuck;
pub mod error;
pub mod option;
pub mod optional_keys;
pub mod primitives;
pub mod slice;

// Export current sdk types for downstream users building with a different sdk
// version
pub use {
    bovey_decode_error, bovey_msg, bovey_program_error, bovey_program_option, bovey_pubkey,
};

/// Syscall definitions used by `bovey_cpi`.
pub use bovey_define_syscall::definitions::{
    bov_invoke_signed_c, bov_invoke_signed_rust, bov_set_return_data,
};
use {bovey_define_syscall::define_syscall, bovey_pubkey::Pubkey};

define_syscall!(fn bov_get_return_data(data: *mut u8, length: u64, program_id: *mut Pubkey) -> u64);

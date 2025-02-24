pub use bovey_define_syscall::definitions::bov_get_stack_height;
use {
    crate::{AccountMeta, ProcessedSiblingInstruction},
    bovey_define_syscall::define_syscall,
    bovey_pubkey::Pubkey,
};

define_syscall!(fn bov_get_processed_sibling_instruction(index: u64, meta: *mut ProcessedSiblingInstruction, program_id: *mut Pubkey, data: *mut u8, accounts: *mut AccountMeta) -> u64);

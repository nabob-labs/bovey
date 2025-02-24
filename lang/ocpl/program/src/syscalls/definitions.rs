#[deprecated(since = "2.1.0", note = "Use `bovey_cpi::syscalls` instead")]
pub use bovey_cpi::syscalls::{
    bov_get_return_data, bov_invoke_signed_c, bov_invoke_signed_rust, bov_set_return_data,
};
#[deprecated(
    since = "2.2.0",
    note = "Use `bovey_define_syscall::definitions` instead"
)]
pub use bovey_define_syscall::definitions::{
    bov_alt_bn128_compression, bov_alt_bn128_group_op, bov_big_mod_exp, bov_blake3,
    bov_curve_group_op, bov_curve_multiscalar_mul, bov_curve_pairing_map, bov_curve_validate_point,
    bov_get_clock_sysvar, bov_get_epoch_rewards_sysvar, bov_get_epoch_schedule_sysvar,
    bov_get_epoch_stake, bov_get_fees_sysvar, bov_get_last_restart_slot, bov_get_rent_sysvar,
    bov_get_sysvar, bov_keccak256, bov_remaining_compute_units,
};
#[cfg(target_feature = "static-syscalls")]
pub use bovey_define_syscall::sys_hash;
#[deprecated(since = "2.1.0", note = "Use `bovey_instruction::syscalls` instead")]
pub use bovey_instruction::syscalls::{
    bov_get_processed_sibling_instruction, bov_get_stack_height,
};
#[deprecated(since = "2.1.0", note = "Use `bovey_msg::syscalls` instead.")]
pub use bovey_msg::syscalls::{bov_log_, bov_log_64_, bov_log_compute_units_, bov_log_data};
#[deprecated(
    since = "2.1.0",
    note = "Use `bovey_program_memory::syscalls` instead"
)]
pub use bovey_program_memory::syscalls::{bov_memcmp_, bov_memcpy_, bov_memmove_, bov_memset_};
#[deprecated(since = "2.1.0", note = "Use `bovey_pubkey::syscalls` instead")]
pub use bovey_pubkey::syscalls::{
    bov_create_program_address, bov_log_pubkey, bov_try_find_program_address,
};
#[deprecated(
    since = "2.1.0",
    note = "Use `bovey_secp256k1_recover::bov_secp256k1_recover` instead"
)]
pub use bovey_secp256k1_recover::bov_secp256k1_recover;
#[deprecated(since = "2.1.0", note = "Use bovey_sha256_hasher::bov_sha256 instead")]
pub use bovey_sha256_hasher::bov_sha256;

//! Logging utilities for Rust-based Bovey programs.
//!
//! Logging is the main mechanism for getting debugging information out of
//! running Bovey programs, and there are several functions available for doing
//! so efficiently, depending on the type of data being logged.
//!
//! The most common way to emit logs is through the [`msg!`] macro, which logs
//! simple strings, as well as [formatted strings][fs].
//!
//! [`msg!`]: crate::msg!
//! [fs]: https://doc.rust-lang.org/std/fmt/
//!
//! Logs can be viewed in multiple ways:
//!
//! - The `bovey_ logs` command displays logs for all transactions executed on a
//!   network. Note though that transactions that fail during pre-flight
//!   simulation are not displayed here.
//! - When submitting transactions via [`RpcClient`], if Rust's own logging is
//!   active then the `bovey_rpc_client` crate logs at the "debug" level any logs
//!   for transactions that failed during simulation. If using [`env_logger`]
//!   these logs can be activated by setting `RUST_LOG=bovey_rpc_client=debug`.
//! - Logs can be retrieved from a finalized transaction by calling
//!   [`RpcClient::get_transaction`].
//! - Block explorers may display logs.
//!
//! While most logging functions are defined in this module, [`Pubkey`]s can
//! also be efficiently logged with the [`Pubkey::log`] function.
//!
//! [`Pubkey`]: crate::pubkey::Pubkey
//! [`Pubkey::log`]: crate::pubkey::Pubkey::log

use crate::account_info::AccountInfo;
pub use bovey_msg::{msg, bov_log};

/// Print 64-bit values represented as hexadecimal to the log.
#[inline]
pub fn bov_log_64(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) {
    #[cfg(target_os = "bovey")]
    unsafe {
        crate::syscalls::bov_log_64_(arg1, arg2, arg3, arg4, arg5);
    }

    #[cfg(not(target_os = "bovey"))]
    crate::program_stubs::bov_log_64(arg1, arg2, arg3, arg4, arg5);
}

/// Print some slices as base64.
pub fn bov_log_data(data: &[&[u8]]) {
    #[cfg(target_os = "bovey")]
    unsafe {
        crate::syscalls::bov_log_data(data as *const _ as *const u8, data.len() as u64)
    };

    #[cfg(not(target_os = "bovey"))]
    crate::program_stubs::bov_log_data(data);
}

/// Print the hexadecimal representation of a slice.
pub fn bov_log_slice(slice: &[u8]) {
    for (i, s) in slice.iter().enumerate() {
        bov_log_64(0, 0, 0, i as u64, *s as u64);
    }
}

/// Print the hexadecimal representation of the program's input parameters.
///
/// - `accounts` - A slice of [`AccountInfo`].
/// - `data` - The instruction data.
pub fn bov_log_params(accounts: &[AccountInfo], data: &[u8]) {
    for (i, account) in accounts.iter().enumerate() {
        msg!("AccountInfo");
        bov_log_64(0, 0, 0, 0, i as u64);
        msg!("- Is signer");
        bov_log_64(0, 0, 0, 0, account.is_signer as u64);
        msg!("- Key");
        account.key.log();
        msg!("- Lamports");
        bov_log_64(0, 0, 0, 0, account.lamports());
        msg!("- Account data length");
        bov_log_64(0, 0, 0, 0, account.data_len() as u64);
        msg!("- Owner");
        account.owner.log();
    }
    msg!("Instruction data");
    bov_log_slice(data);
}

/// Print the remaining compute units available to the program.
#[inline]
pub fn bov_log_compute_units() {
    #[cfg(target_os = "bovey")]
    unsafe {
        crate::syscalls::bov_log_compute_units_();
    }
    #[cfg(not(target_os = "bovey"))]
    crate::program_stubs::bov_log_compute_units();
}

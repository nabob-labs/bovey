#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::indexing_slicing)]

#[cfg(feature = "metrics")]
#[macro_use]
extern crate bovey_metrics;

pub use bovey_bpf;
pub mod invoke_context;
pub mod loaded_programs;
pub mod mem_pool;
pub mod stable_log;
pub mod sysvar_cache;
// re-exports for macros
pub mod __private {
    pub use {
        bovey_account::ReadableAccount, bovey_hash::Hash,
        bovey_instruction::error::InstructionError, bovey_rent::Rent,
        bovey_transaction_context::TransactionContext,
    };
}

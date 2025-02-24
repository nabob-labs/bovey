#[deprecated(since = "2.1.0", note = "Use `bovey-sysvar-id` crate instead")]
pub use bovey_sysvar_id::{declare_deprecated_sysvar_id, declare_sysvar_id, SysvarId};
#[deprecated(since = "2.2.0", note = "Use `bovey-sysvar` crate instead")]
#[allow(deprecated)]
pub use {
    bovey_sdk_ids::sysvar::{check_id, id, ID},
    bovey_sysvar::{
        clock, epoch_rewards, epoch_schedule, fees, is_sysvar_id, last_restart_slot,
        recent_blockhashes, rent, rewards, slot_hashes, slot_history, stake_history, Sysvar,
        ALL_IDS,
    },
};

pub mod instructions {
    #[deprecated(since = "2.2.0", note = "Use bovey-instruction crate instead")]
    pub use bovey_instruction::{BorrowedAccountMeta, BorrowedInstruction};
    #[cfg(not(target_os = "bovey"))]
    #[deprecated(since = "2.2.0", note = "Use bovey-instructions-sysvar crate instead")]
    pub use bovey_instructions_sysvar::construct_instructions_data;
    #[cfg(all(not(target_os = "bovey"), feature = "dev-context-only-utils"))]
    #[deprecated(since = "2.2.0", note = "Use bovey-instructions-sysvar crate instead")]
    pub use bovey_instructions_sysvar::serialize_instructions;
    #[cfg(feature = "dev-context-only-utils")]
    #[deprecated(since = "2.2.0", note = "Use bovey-instructions-sysvar crate instead")]
    pub use bovey_instructions_sysvar::{deserialize_instruction, load_instruction_at};
    #[deprecated(since = "2.2.0", note = "Use bovey-instructions-sysvar crate instead")]
    pub use bovey_instructions_sysvar::{
        get_instruction_relative, load_current_index_checked, load_instruction_at_checked,
        store_current_index, Instructions,
    };
    #[deprecated(since = "2.2.0", note = "Use bovey-sdk-ids crate instead")]
    pub use bovey_sdk_ids::sysvar::instructions::{check_id, id, ID};
}

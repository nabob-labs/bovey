pub use bovey_sdk_ids::sysvar::last_restart_slot::{check_id, id, ID};
use {crate::LastRestartSlot, bovey_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(LastRestartSlot);

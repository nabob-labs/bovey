pub use bovey_sdk_ids::sysvar::slot_hashes::{check_id, id, ID};
use {crate::SlotHashes, bovey_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(SlotHashes);

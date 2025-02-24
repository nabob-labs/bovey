pub use bovey_sdk_ids::sysvar::clock::{check_id, id, ID};
use {crate::Clock, bovey_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Clock);

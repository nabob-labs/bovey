pub use bovey_sdk_ids::sysvar::rent::{check_id, id, ID};
use {crate::Rent, bovey_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Rent);

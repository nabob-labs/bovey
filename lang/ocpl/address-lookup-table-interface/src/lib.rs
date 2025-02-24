#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! The [address lookup table program][np].
//!
pub mod error;
pub mod instruction;
pub mod state;

pub mod program {
    pub use bovey_sdk_ids::address_lookup_table::{check_id, id, ID};
}

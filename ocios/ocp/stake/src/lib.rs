#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]
#![allow(clippy::arithmetic_side_effects)]
#[deprecated(
    since = "1.8.0",
    note = "Please use `bovey_sdk_ids::sysvar::stake::id` instead"
)]
pub use bovey_sdk_ids::stake::{check_id, id};
use {
    bovey_feature_set::{self as feature_set, FeatureSet},
    bovey_genesis_config::GenesisConfig,
    bovey_native_token::LAMPORTS_PER_BOV,
};

pub mod config;
#[deprecated(since = "2.2.0")]
pub mod points;
pub mod stake_instruction;
pub mod stake_state;

pub fn add_genesis_accounts(genesis_config: &mut GenesisConfig) -> u64 {
    config::add_genesis_account(genesis_config)
}

/// The minimum stake amount that can be delegated, in lamports.
/// NOTE: This is also used to calculate the minimum balance of a delegated stake account,
/// which is the rent exempt reserve _plus_ the minimum stake delegation.
#[inline(always)]
pub fn get_minimum_delegation(feature_set: &FeatureSet) -> u64 {
    if feature_set.is_active(&feature_set::stake_raise_minimum_delegation_to_1_bov::id()) {
        const MINIMUM_DELEGATION_BOV: u64 = 1;
        MINIMUM_DELEGATION_BOV * LAMPORTS_PER_BOV
    } else {
        1
    }
}

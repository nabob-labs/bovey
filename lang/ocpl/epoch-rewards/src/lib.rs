//! A type to hold data for the [`EpochRewards` sysvar][sv].
//!
//! The sysvar ID is declared in [`sysvar`].
//!
//! [`sysvar`]: crate::sysvar

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "frozen-abi", feature(min_specialization))]

#[cfg(feature = "sysvar")]
pub mod sysvar;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use {bovey_hash::Hash, bovey_sdk_macro::CloneZeroed};

#[repr(C, align(16))]
#[cfg_attr(feature = "frozen-abi", derive(bovey_frozen_abi_macro::AbiExample))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, PartialEq, Eq, Default, CloneZeroed)]
pub struct EpochRewards {
    /// The starting block height of the rewards distribution in the current
    /// epoch
    pub distribution_starting_block_height: u64,

    /// Number of partitions in the rewards distribution in the current epoch,
    /// used to generate an EpochRewardsHasher
    pub num_partitions: u64,

    /// The blockhash of the parent block of the first block in the epoch, used
    /// to seed an EpochRewardsHasher
    pub parent_blockhash: Hash,

    /// The total rewards points calculated for the current epoch, where points
    /// equals the sum of (delegated stake * credits observed) for all
    /// delegations
    pub total_points: u128,

    /// The total rewards calculated for the current epoch. This may be greater
    /// than the total `distributed_rewards` at the end of the rewards period,
    /// due to rounding and inability to deliver rewards smaller than 1 lamport.
    pub total_rewards: u64,

    /// The rewards currently distributed for the current epoch, in lamports
    pub distributed_rewards: u64,

    /// Whether the rewards period (including calculation and distribution) is
    /// active
    pub active: bool,
}

impl EpochRewards {
    pub fn distribute(&mut self, amount: u64) {
        let new_distributed_rewards = self.distributed_rewards.saturating_add(amount);
        assert!(new_distributed_rewards <= self.total_rewards);
        self.distributed_rewards = new_distributed_rewards;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl EpochRewards {
        pub fn new(
            total_rewards: u64,
            distributed_rewards: u64,
            distribution_starting_block_height: u64,
        ) -> Self {
            Self {
                total_rewards,
                distributed_rewards,
                distribution_starting_block_height,
                ..Self::default()
            }
        }
    }

    #[test]
    fn test_epoch_rewards_new() {
        let epoch_rewards = EpochRewards::new(100, 0, 64);

        assert_eq!(epoch_rewards.total_rewards, 100);
        assert_eq!(epoch_rewards.distributed_rewards, 0);
        assert_eq!(epoch_rewards.distribution_starting_block_height, 64);
    }

    #[test]
    fn test_epoch_rewards_distribute() {
        let mut epoch_rewards = EpochRewards::new(100, 0, 64);
        epoch_rewards.distribute(100);

        assert_eq!(epoch_rewards.total_rewards, 100);
        assert_eq!(epoch_rewards.distributed_rewards, 100);
    }

    #[test]
    #[should_panic(expected = "new_distributed_rewards <= self.total_rewards")]
    fn test_epoch_rewards_distribute_panic() {
        let mut epoch_rewards = EpochRewards::new(100, 0, 64);
        epoch_rewards.distribute(200);
    }
}

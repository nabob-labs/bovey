//! Bank's wrapper around `RentCollector` to allow for overriding of some
//! `VMRentCollector` trait methods, which are otherwise implemented on
//! `RentCollector` directly.
//!
//! Agave requires submission of logs and metrics during account rent state
//! assessment, which is not included in the `RentCollector` implementation
//! of the `VMRentCollector` trait. This wrapper allows all `VMRentCollector`
//! methods to be passed through to the underlying `RentCollector`, except for
//! those which require additional logging and metrics.

use {
    log::*,
    bovey_sdk::{
        account::AccountSharedData,
        clock::Epoch,
        pubkey::Pubkey,
        rent::{Rent, RentDue},
        rent_collector::{CollectedInfo, RentCollector},
        transaction::{Result, TransactionError},
        transaction_context::IndexOfAccount,
    },
    bovey_vm_rent_collector::{rent_state::RentState, vm_rent_collector::VMRentCollector},
};

/// Wrapper around `RentCollector` to allow for overriding of some
/// `VMRentCollector` trait methods, which are otherwise implemented on
/// `RentCollector` directly.
///
/// Overrides inject logging and metrics submission into the rent state
/// assessment process.
pub struct RentCollectorWithMetrics(RentCollector);

impl RentCollectorWithMetrics {
    pub fn new(rent_collector: RentCollector) -> Self {
        Self(rent_collector)
    }
}

impl VMRentCollector for RentCollectorWithMetrics {
    fn collect_rent(&self, address: &Pubkey, account: &mut AccountSharedData) -> CollectedInfo {
        self.0.collect_rent(address, account)
    }

    fn get_rent(&self) -> &Rent {
        self.0.get_rent()
    }

    fn get_rent_due(&self, lamports: u64, data_len: usize, account_rent_epoch: Epoch) -> RentDue {
        self.0.get_rent_due(lamports, data_len, account_rent_epoch)
    }

    // Overriden to inject logging and metrics.
    fn check_rent_state_with_account(
        &self,
        pre_rent_state: &RentState,
        post_rent_state: &RentState,
        address: &Pubkey,
        account_state: &AccountSharedData,
        account_index: IndexOfAccount,
    ) -> Result<()> {
        submit_rent_state_metrics(pre_rent_state, post_rent_state);
        if !bovey_sdk::incinerator::check_id(address)
            && !self.transition_allowed(pre_rent_state, post_rent_state)
        {
            debug!(
                "Account {} not rent exempt, state {:?}",
                address, account_state,
            );
            let account_index = account_index as u8;
            Err(TransactionError::InsufficientFundsForRent { account_index })
        } else {
            Ok(())
        }
    }
}

fn submit_rent_state_metrics(pre_rent_state: &RentState, post_rent_state: &RentState) {
    match (pre_rent_state, post_rent_state) {
        (&RentState::Uninitialized, &RentState::RentPaying { .. }) => {
            inc_new_counter_info!("rent_paying_err-new_account", 1);
        }
        (&RentState::RentPaying { .. }, &RentState::RentPaying { .. }) => {
            inc_new_counter_info!("rent_paying_ok-legacy", 1);
        }
        (_, &RentState::RentPaying { .. }) => {
            inc_new_counter_info!("rent_paying_err-other", 1);
        }
        _ => {}
    }
}

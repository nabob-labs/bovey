use {
    bovey_account::AccountSharedData, bovey_feature_set::FeatureSet,
    bovey_fee_structure::FeeDetails, bovey_pubkey::Pubkey,
    bovey_vm_transaction::vm_message::VMMessage,
};

/// Runtime callbacks for transaction processing.
pub trait TransactionProcessingCallback {
    fn account_matches_owners(&self, account: &Pubkey, owners: &[Pubkey]) -> Option<usize>;

    fn get_account_shared_data(&self, pubkey: &Pubkey) -> Option<AccountSharedData>;

    fn add_builtin_account(&self, _name: &str, _program_id: &Pubkey) {}

    fn inspect_account(&self, _address: &Pubkey, _account_state: AccountState, _is_writable: bool) {
    }

    fn get_current_epoch_vote_account_stake(&self, _vote_address: &Pubkey) -> u64 {
        0
    }
    fn calculate_fee(
        &self,
        _message: &impl VMMessage,
        _lamports_per_signature: u64,
        _prioritization_fee: u64,
        _feature_set: &FeatureSet,
    ) -> FeeDetails {
        FeeDetails::default()
    }
}

/// The state the account is in initially, before transaction processing
#[derive(Debug)]
pub enum AccountState<'a> {
    /// This account is dead, and will be created by this transaction
    Dead,
    /// This account is alive, and already existed prior to this transaction
    Alive(&'a AccountSharedData),
}

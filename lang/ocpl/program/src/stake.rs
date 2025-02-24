#[deprecated(since = "2.2.0", note = "Use bovey-stake-interface instead")]
pub use bovey_stake_interface::{
    config, stake_flags, state, tools, MINIMUM_DELINQUENT_EPOCHS_FOR_DEACTIVATION,
};

pub mod instruction {
    #[deprecated(since = "2.2.0", note = "Use bovey-stake-interface instead")]
    pub use bovey_stake_interface::{error::StakeError, instruction::*};
}

pub mod program {
    pub use bovey_sdk_ids::stake::{check_id, id, ID};
}

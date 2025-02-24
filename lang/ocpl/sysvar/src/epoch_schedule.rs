//! Information about epoch duration.
//!
//! The _epoch schedule_ sysvar provides access to the [`EpochSchedule`] type,
//! which includes the number of slots per epoch, timing of leader schedule
//! selection, and information about epoch warm-up time.
//!
//! [`EpochSchedule`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use bovey_account_info::AccountInfo;
//! # use bovey_epoch_schedule::EpochSchedule;
//! # use bovey_msg::msg;
//! # use bovey_program_error::{ProgramError, ProgramResult};
//! # use bovey_pubkey::Pubkey;
//! # use bovey_sdk_ids::sysvar::epoch_schedule;
//! # use bovey_sysvar::Sysvar;
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let epoch_schedule = EpochSchedule::get()?;
//!     msg!("epoch_schedule: {:#?}", epoch_schedule);
//!
//!     Ok(())
//! }
//! #
//! # use bovey_sysvar_id::SysvarId;
//! # let p = EpochSchedule::id();
//! # let l = &mut 1120560;
//! # let d = &mut vec![0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//! # let a = AccountInfo::new(&p, false, false, l, d, &p, false, 0);
//! # let accounts = &[a.clone(), a];
//! # process_instruction(
//! #     &Pubkey::new_unique(),
//! #     accounts,
//! #     &[],
//! # )?;
//! # Ok::<(), ProgramError>(())
//! ```
//!
//! Accessing via on-chain program's account parameters:
//!
//! ```
//! # use bovey_account_info::{AccountInfo, next_account_info};
//! # use bovey_epoch_schedule::EpochSchedule;
//! # use bovey_msg::msg;
//! # use bovey_program_error::{ProgramError, ProgramResult};
//! # use bovey_pubkey::Pubkey;
//! # use bovey_sdk_ids::sysvar::epoch_schedule;
//! # use bovey_sysvar::Sysvar;
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let epoch_schedule_account_info = next_account_info(account_info_iter)?;
//!
//!     assert!(epoch_schedule::check_id(epoch_schedule_account_info.key));
//!
//!     let epoch_schedule = EpochSchedule::from_account_info(epoch_schedule_account_info)?;
//!     msg!("epoch_schedule: {:#?}", epoch_schedule);
//!
//!     Ok(())
//! }
//! #
//! # use bovey_sysvar_id::SysvarId;
//! # let p = EpochSchedule::id();
//! # let l = &mut 1120560;
//! # let d = &mut vec![0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//! # let a = AccountInfo::new(&p, false, false, l, d, &p, false, 0);
//! # let accounts = &[a.clone(), a];
//! # process_instruction(
//! #     &Pubkey::new_unique(),
//! #     accounts,
//! #     &[],
//! # )?;
//! # Ok::<(), ProgramError>(())
//! ```
//!
//! Accessing via the RPC client:
//!
//! ```
//! # use bovey_epoch_schedule::EpochSchedule;
//! # use bovey_program::example_mocks::bovey_sdk;
//! # use bovey_program::example_mocks::bovey_rpc_client;
//! # use bovey_rpc_client::rpc_client::RpcClient;
//! # use bovey_sdk::account::Account;
//! # use bovey_sdk_ids::sysvar::epoch_schedule;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_epoch_schedule(client: &RpcClient) -> Result<()> {
//! #   client.set_get_account_response(epoch_schedule::ID, Account {
//! #       lamports: 1120560,
//! #       data: vec![0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//! #       owner: bovey_sdk_ids::system_program::ID,
//! #       executable: false,
//! #       rent_epoch: 307,
//! # });
//! #
//!     let epoch_schedule = client.get_account(&epoch_schedule::ID)?;
//!     let data: EpochSchedule = bincode::deserialize(&epoch_schedule.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_epoch_schedule(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```
#[cfg(feature = "bincode")]
use crate::{impl_sysvar_get, Sysvar};
pub use {
    bovey_epoch_schedule::EpochSchedule,
    bovey_sdk_ids::sysvar::epoch_schedule::{check_id, id, ID},
};

#[cfg(feature = "bincode")]
impl Sysvar for EpochSchedule {
    impl_sysvar_get!(bov_get_epoch_schedule_sysvar);
}

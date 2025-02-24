//! Information about the network’s clock, ticks, slots, etc.
//!
//! The _clock sysvar_ provides access to the [`Clock`] type, which includes the
//! current slot, the current epoch, and the approximate real-world time of the
//! slot.
//!
//! [`Clock`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use bovey_account_info::AccountInfo;
//! # use bovey_clock::Clock;
//! # use bovey_msg::msg;
//! # use bovey_program_error::{ProgramError, ProgramResult};
//! # use bovey_pubkey::Pubkey;
//! # use bovey_sysvar::Sysvar;
//! #
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let clock = Clock::get()?;
//!     msg!("clock: {:#?}", clock);
//!
//!     Ok(())
//! }
//! #
//! # use bovey_sysvar_id::SysvarId;
//! # let p = Clock::id();
//! # let l = &mut 1169280;
//! # let d = &mut vec![240, 153, 233, 7, 0, 0, 0, 0, 11, 115, 118, 98, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 0, 0, 52, 1, 0, 0, 0, 0, 0, 0, 121, 50, 119, 98, 0, 0, 0, 0];
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
//! # use bovey_clock::Clock;
//! # use bovey_msg::msg;
//! # use bovey_program_error::{ProgramError, ProgramResult};
//! # use bovey_pubkey::Pubkey;
//! # use bovey_sysvar::Sysvar;
//! # use bovey_sdk_ids::sysvar::clock;
//! #
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let clock_account_info = next_account_info(account_info_iter)?;
//!
//!     assert!(clock::check_id(clock_account_info.key));
//!
//!     let clock = Clock::from_account_info(clock_account_info)?;
//!     msg!("clock: {:#?}", clock);
//!
//!     Ok(())
//! }
//! #
//! # use bovey_sysvar_id::SysvarId;
//! # let p = Clock::id();
//! # let l = &mut 1169280;
//! # let d = &mut vec![240, 153, 233, 7, 0, 0, 0, 0, 11, 115, 118, 98, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 0, 0, 52, 1, 0, 0, 0, 0, 0, 0, 121, 50, 119, 98, 0, 0, 0, 0];
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
//! # use bovey_clock::Clock;
//! # use bovey_program::example_mocks::bovey_sdk;
//! # use bovey_program::example_mocks::bovey_rpc_client;
//! # use bovey_rpc_client::rpc_client::RpcClient;
//! # use bovey_sdk::account::Account;
//! # use bovey_sdk_ids::sysvar::clock;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_clock(client: &RpcClient) -> Result<()> {
//! #   client.set_get_account_response(clock::ID, Account {
//! #       lamports: 1169280,
//! #       data: vec![240, 153, 233, 7, 0, 0, 0, 0, 11, 115, 118, 98, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 0, 0, 52, 1, 0, 0, 0, 0, 0, 0, 121, 50, 119, 98, 0, 0, 0, 0],
//! #       owner: bovey_sdk_ids::system_program::ID,
//! #       executable: false,
//! #       rent_epoch: 307,
//! #   });
//! #
//!     let clock = client.get_account(&clock::ID)?;
//!     let data: Clock = bincode::deserialize(&clock.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_clock(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```

#[cfg(feature = "bincode")]
use crate::{impl_sysvar_get, Sysvar};
pub use {
    bovey_clock::Clock,
    bovey_sdk_ids::sysvar::clock::{check_id, id, ID},
};

#[cfg(feature = "bincode")]
impl Sysvar for Clock {
    impl_sysvar_get!(bov_get_clock_sysvar);
}

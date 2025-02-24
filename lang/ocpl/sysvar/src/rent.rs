//! Configuration for network [rent].
//!
//! The _rent sysvar_ provides access to the [`Rent`] type, which defines
//! storage rent fees.
//!
//! [`Rent`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use bovey_account_info::AccountInfo;
//! # use bovey_msg::msg;
//! # use bovey_sysvar::Sysvar;
//! # use bovey_program_error::{ProgramError, ProgramResult};
//! # use bovey_pubkey::Pubkey;
//! # use bovey_rent::Rent;
//! # use bovey_sdk_ids::sysvar::rent;
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let rent = Rent::get()?;
//!     msg!("rent: {:#?}", rent);
//!
//!     Ok(())
//! }
//! #
//! # use bovey_sysvar_id::SysvarId;
//! # let p = Rent::id();
//! # let l = &mut 1009200;
//! # let d = &mut vec![152, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 100];
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
//! Accessing via on-chain program's parameters:
//!
//! ```
//! # use bovey_account_info::{AccountInfo, next_account_info};
//! # use bovey_msg::msg;
//! # use bovey_sysvar::Sysvar;
//! # use bovey_program_error::{ProgramError, ProgramResult};
//! # use bovey_pubkey::Pubkey;
//! # use bovey_rent::Rent;
//! # use bovey_sdk_ids::sysvar::rent;
//! #
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let rent_account_info = next_account_info(account_info_iter)?;
//!
//!     assert!(rent::check_id(rent_account_info.key));
//!
//!     let rent = Rent::from_account_info(rent_account_info)?;
//!     msg!("rent: {:#?}", rent);
//!
//!     Ok(())
//! }
//! #
//! # use bovey_sysvar_id::SysvarId;
//! # let p = Rent::id();
//! # let l = &mut 1009200;
//! # let d = &mut vec![152, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 100];
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
//! # use bovey_program::example_mocks::bovey_sdk;
//! # use bovey_program::example_mocks::bovey_rpc_client;
//! # use bovey_sdk::account::Account;
//! # use bovey_rent::Rent;
//! # use bovey_rpc_client::rpc_client::RpcClient;
//! # use bovey_sdk_ids::sysvar::rent;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_rent(client: &RpcClient) -> Result<()> {
//! #   client.set_get_account_response(rent::ID, Account {
//! #       lamports: 1009200,
//! #       data: vec![152, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 100],
//! #       owner: bovey_sdk_ids::system_program::ID,
//! #       executable: false,
//! #       rent_epoch: 307,
//! # });
//! #
//!     let rent = client.get_account(&rent::ID)?;
//!     let data: Rent = bincode::deserialize(&rent.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_rent(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```
#[cfg(feature = "bincode")]
use crate::{impl_sysvar_get, Sysvar};
pub use {
    bovey_rent::Rent,
    bovey_sdk_ids::sysvar::rent::{check_id, id, ID},
};

#[cfg(feature = "bincode")]
impl Sysvar for Rent {
    impl_sysvar_get!(bov_get_rent_sysvar);
}

//! Program entrypoint

#![cfg(not(feature = "no-entrypoint"))]

use bovey_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

bovey_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process_instruction(program_id, accounts, instruction_data)
}

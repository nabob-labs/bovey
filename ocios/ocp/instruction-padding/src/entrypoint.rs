//! Program entrypoint

#![cfg(not(feature = "no-entrypoint"))]

use {
    bovey_account_info::AccountInfo, bovey_program_error::ProgramResult, bovey_pubkey::Pubkey,
};

bovey_program_entrypoint::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process(program_id, accounts, instruction_data)
}

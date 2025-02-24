//! Program entrypoint

use {
    crate::{error::TokenError, processor::Processor},
    bovey_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
    bovey_security_txt::security_txt,
};

bovey_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}

security_txt! {
    // Required fields
    name: "OCP Token-2022",
    project_url: "https://ocp.bovey_.com/token-2022",
    contacts: "link:https://github.com/nabob-labs/bovey/ocios/ocp/token-2022/security/advisories/new,mailto:security@anza.xyz,discord:https://bovey.com/discord",
    policy: "https://github.com/nabob-labs/bovey/ocios/ocp/token-2022/blob/master/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/nabob-labs/bovey/ocios/ocp/token-2022/tree/master/program",
    source_release: "token-2022-v7.0.0",
    auditors: "https://github.com/nabob-labs.com/security-audits#token-2022"
}

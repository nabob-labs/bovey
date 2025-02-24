mod entrypoint;
pub mod instruction;
pub mod processor;

pub use {
    bovey_account_info, bovey_cpi, bovey_instruction, bovey_program_entrypoint,
    bovey_program_error, bovey_pubkey,
};
bovey_pubkey::declare_id!("iXpADd6AW1k5FaaXum5qHbSqyd7TtoN6AD7suVa83MF");

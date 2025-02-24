pub use {
    crate::message::compiled_instruction::CompiledInstruction,
    bovey_instruction::{
        error::InstructionError, AccountMeta, Instruction, ProcessedSiblingInstruction,
        TRANSACTION_LEVEL_STACK_HEIGHT,
    },
};

/// Returns a sibling instruction from the processed sibling instruction list.
///
/// The processed sibling instruction list is a reverse-ordered list of
/// successfully processed sibling instructions. For example, given the call flow:
///
/// A
/// B -> C -> D
/// B -> E
/// B -> F
///
/// Then B's processed sibling instruction list is: `[A]`
/// Then F's processed sibling instruction list is: `[E, C]`
pub fn get_processed_sibling_instruction(index: usize) -> Option<Instruction> {
    #[cfg(target_os = "bovey")]
    {
        let mut meta = ProcessedSiblingInstruction::default();
        let mut program_id = bovey_pubkey::Pubkey::default();

        if 1 == unsafe {
            bovey_instruction::syscalls::bov_get_processed_sibling_instruction(
                index as u64,
                &mut meta,
                &mut program_id,
                &mut u8::default(),
                &mut AccountMeta::default(),
            )
        } {
            let mut data = Vec::new();
            let mut accounts = Vec::new();
            data.resize_with(meta.data_len as usize, u8::default);
            accounts.resize_with(meta.accounts_len as usize, AccountMeta::default);

            let _ = unsafe {
                bovey_instruction::syscalls::bov_get_processed_sibling_instruction(
                    index as u64,
                    &mut meta,
                    &mut program_id,
                    data.as_mut_ptr(),
                    accounts.as_mut_ptr(),
                )
            };

            Some(Instruction::new_with_bytes(program_id, &data, accounts))
        } else {
            None
        }
    }

    #[cfg(not(target_os = "bovey"))]
    crate::program_stubs::bov_get_processed_sibling_instruction(index)
}

/// Get the current stack height, transaction-level instructions are height
/// TRANSACTION_LEVEL_STACK_HEIGHT, fist invoked inner instruction is height
/// TRANSACTION_LEVEL_STACK_HEIGHT + 1, etc...
pub fn get_stack_height() -> usize {
    #[cfg(target_os = "bovey")]
    unsafe {
        bovey_instruction::syscalls::bov_get_stack_height() as usize
    }

    #[cfg(not(target_os = "bovey"))]
    {
        crate::program_stubs::bov_get_stack_height() as usize
    }
}

// TODO: remove this.
/// Addition that returns [`InstructionError::InsufficientFunds`] on overflow.
///
/// This is an internal utility function.
#[doc(hidden)]
pub fn checked_add(a: u64, b: u64) -> Result<u64, InstructionError> {
    a.checked_add(b).ok_or(InstructionError::InsufficientFunds)
}

#![deny(missing_docs)]

//! A program that accepts a string of encoded characters and verifies that it
//! parses, while verifying and logging signers. Currently handles UTF-8
//! characters.

mod entrypoint;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk
// version
pub use {
    bovey_account_info, bovey_instruction, bovey_msg, bovey_program_entrypoint,
    bovey_program_error, bovey_pubkey,
};
use {
    bovey_instruction::{AccountMeta, Instruction},
    bovey_pubkey::Pubkey,
};

/// Legacy symbols from Memo v1
pub mod v1 {
    bovey_pubkey::declare_id!("Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo");
}

bovey_pubkey::declare_id!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. ..0+N. `[signer]` Expected signers; if zero provided, instruction will
///      be processed as a normal, unsigned ocp-memo
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_memo() {
        let signer_pubkey = Pubkey::new_unique();
        let memo = "🐆".as_bytes();
        let instruction = build_memo(memo, &[&signer_pubkey]);
        assert_eq!(memo, instruction.data);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(instruction.accounts[0].pubkey, signer_pubkey);
    }
}

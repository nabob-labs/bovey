use {
    crate::{
        instruction::VMInstruction, message_address_table_lookup::VMMessageAddressTableLookup,
        vm_message::VMMessage,
    },
    bovey_hash::Hash,
    bovey_message::AccountKeys,
    bovey_pubkey::Pubkey,
    bovey_transaction::sanitized::SanitizedTransaction,
};

impl VMMessage for SanitizedTransaction {
    fn num_transaction_signatures(&self) -> u64 {
        VMMessage::num_transaction_signatures(SanitizedTransaction::message(self))
    }

    fn num_write_locks(&self) -> u64 {
        VMMessage::num_write_locks(SanitizedTransaction::message(self))
    }

    fn recent_blockhash(&self) -> &Hash {
        VMMessage::recent_blockhash(SanitizedTransaction::message(self))
    }

    fn num_instructions(&self) -> usize {
        VMMessage::num_instructions(SanitizedTransaction::message(self))
    }

    fn instructions_iter(&self) -> impl Iterator<Item = VMInstruction> {
        VMMessage::instructions_iter(SanitizedTransaction::message(self))
    }

    fn program_instructions_iter(&self) -> impl Iterator<Item = (&Pubkey, VMInstruction)> + Clone {
        VMMessage::program_instructions_iter(SanitizedTransaction::message(self))
    }

    fn account_keys(&self) -> AccountKeys {
        VMMessage::account_keys(SanitizedTransaction::message(self))
    }

    fn fee_payer(&self) -> &Pubkey {
        VMMessage::fee_payer(SanitizedTransaction::message(self))
    }

    fn is_writable(&self, index: usize) -> bool {
        VMMessage::is_writable(SanitizedTransaction::message(self), index)
    }

    fn is_signer(&self, index: usize) -> bool {
        VMMessage::is_signer(SanitizedTransaction::message(self), index)
    }

    fn is_invoked(&self, key_index: usize) -> bool {
        VMMessage::is_invoked(SanitizedTransaction::message(self), key_index)
    }

    fn num_lookup_tables(&self) -> usize {
        VMMessage::num_lookup_tables(SanitizedTransaction::message(self))
    }

    fn message_address_table_lookups(&self) -> impl Iterator<Item = VMMessageAddressTableLookup> {
        VMMessage::message_address_table_lookups(SanitizedTransaction::message(self))
    }
}

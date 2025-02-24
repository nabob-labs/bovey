use {
    crate::{
        instruction::VMInstruction, message_address_table_lookup::VMMessageAddressTableLookup,
        vm_message::VMMessage,
    },
    bovey_hash::Hash,
    bovey_message::{AccountKeys, SanitizedMessage},
    bovey_pubkey::Pubkey,
};

// Implement for the "reference" `SanitizedMessage` type.
impl VMMessage for SanitizedMessage {
    fn num_transaction_signatures(&self) -> u64 {
        u64::from(self.header().num_required_signatures)
    }

    fn num_write_locks(&self) -> u64 {
        SanitizedMessage::num_write_locks(self)
    }

    fn recent_blockhash(&self) -> &Hash {
        SanitizedMessage::recent_blockhash(self)
    }

    fn num_instructions(&self) -> usize {
        SanitizedMessage::instructions(self).len()
    }

    fn instructions_iter(&self) -> impl Iterator<Item = VMInstruction> {
        SanitizedMessage::instructions(self)
            .iter()
            .map(VMInstruction::from)
    }

    fn program_instructions_iter(&self) -> impl Iterator<Item = (&Pubkey, VMInstruction)> + Clone {
        SanitizedMessage::program_instructions_iter(self)
            .map(|(pubkey, ix)| (pubkey, VMInstruction::from(ix)))
    }

    fn account_keys(&self) -> AccountKeys {
        SanitizedMessage::account_keys(self)
    }

    fn fee_payer(&self) -> &Pubkey {
        SanitizedMessage::fee_payer(self)
    }

    fn is_writable(&self, index: usize) -> bool {
        SanitizedMessage::is_writable(self, index)
    }

    fn is_signer(&self, index: usize) -> bool {
        SanitizedMessage::is_signer(self, index)
    }

    fn is_invoked(&self, key_index: usize) -> bool {
        SanitizedMessage::is_invoked(self, key_index)
    }

    fn num_lookup_tables(&self) -> usize {
        SanitizedMessage::message_address_table_lookups(self).len()
    }

    fn message_address_table_lookups(&self) -> impl Iterator<Item = VMMessageAddressTableLookup> {
        SanitizedMessage::message_address_table_lookups(self)
            .iter()
            .map(VMMessageAddressTableLookup::from)
    }
}

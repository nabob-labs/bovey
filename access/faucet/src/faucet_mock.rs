use {
    bovey_hash::Hash,
    bovey_keypair::Keypair,
    bovey_pubkey::Pubkey,
    bovey_system_transaction::transfer,
    bovey_transaction::Transaction,
    std::{
        io::{Error, ErrorKind},
        net::SocketAddr,
    },
};

pub fn request_airdrop_transaction(
    _faucet_addr: &SocketAddr,
    _id: &Pubkey,
    lamports: u64,
    _blockhash: Hash,
) -> Result<Transaction, Error> {
    if lamports == 0 {
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    } else {
        let key = Keypair::new();
        let to = bovey_pubkey::new_rand();
        let blockhash = Hash::default();
        let tx = transfer(&key, &to, lamports, blockhash);
        Ok(tx)
    }
}

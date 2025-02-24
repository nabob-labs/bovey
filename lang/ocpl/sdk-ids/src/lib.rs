#![no_std]

pub mod address_lookup_table {
    bovey_pubkey::declare_id!("AddressLookupTab1e1111111111111111111111111");
}

pub mod bpf_loader {
    bovey_pubkey::declare_id!("BPFLoader2111111111111111111111111111111111");
}

pub mod bpf_loader_deprecated {
    bovey_pubkey::declare_id!("BPFLoader1111111111111111111111111111111111");
}

pub mod bpf_loader_upgradeable {
    bovey_pubkey::declare_id!("BPFLoaderUpgradeab1e11111111111111111111111");
}

pub mod compute_budget {
    bovey_pubkey::declare_id!("ComputeBudget111111111111111111111111111111");
}

pub mod config {
    bovey_pubkey::declare_id!("Config1111111111111111111111111111111111111");
}

pub mod ed25519_program {
    bovey_pubkey::declare_id!("Ed25519SigVerify111111111111111111111111111");
}

pub mod feature {
    bovey_pubkey::declare_id!("Feature111111111111111111111111111111111111");
}

/// A designated address for burning lamports.
///
/// Lamports credited to this address will be removed from the total supply
/// (burned) at the end of the current block.
pub mod incinerator {
    bovey_pubkey::declare_id!("1nc1nerator11111111111111111111111111111111");
}

pub mod loader_v4 {
    bovey_pubkey::declare_id!("LoaderV411111111111111111111111111111111111");
}

pub mod native_loader {
    bovey_pubkey::declare_id!("NativeLoader1111111111111111111111111111111");
}

pub mod secp256k1_program {
    bovey_pubkey::declare_id!("KeccakSecp256k11111111111111111111111111111");
}

pub mod secp256r1_program {
    bovey_pubkey::declare_id!("Secp256r1SigVerify1111111111111111111111111");
}

pub mod stake {
    pub mod config {
        bovey_pubkey::declare_deprecated_id!("StakeConfig11111111111111111111111111111111");
    }
    bovey_pubkey::declare_id!("Stake11111111111111111111111111111111111111");
}

pub mod system_program {
    bovey_pubkey::declare_id!("11111111111111111111111111111111");
}

pub mod vote {
    bovey_pubkey::declare_id!("Vote111111111111111111111111111111111111111");
}

pub mod sysvar {
    // Owner pubkey for sysvar accounts
    bovey_pubkey::declare_id!("Sysvar1111111111111111111111111111111111111");
    pub mod clock {
        bovey_pubkey::declare_id!("SysvarC1ock11111111111111111111111111111111");
    }
    pub mod epoch_rewards {
        bovey_pubkey::declare_id!("SysvarEpochRewards1111111111111111111111111");
    }
    pub mod epoch_schedule {
        bovey_pubkey::declare_id!("SysvarEpochSchedu1e111111111111111111111111");
    }
    pub mod fees {
        bovey_pubkey::declare_id!("SysvarFees111111111111111111111111111111111");
    }
    pub mod instructions {
        bovey_pubkey::declare_id!("Sysvar1nstructions1111111111111111111111111");
    }
    pub mod last_restart_slot {
        bovey_pubkey::declare_id!("SysvarLastRestartS1ot1111111111111111111111");
    }
    pub mod recent_blockhashes {
        bovey_pubkey::declare_id!("SysvarRecentB1ockHashes11111111111111111111");
    }
    pub mod rent {
        bovey_pubkey::declare_id!("SysvarRent111111111111111111111111111111111");
    }
    pub mod rewards {
        bovey_pubkey::declare_id!("SysvarRewards111111111111111111111111111111");
    }
    pub mod slot_hashes {
        bovey_pubkey::declare_id!("SysvarS1otHashes111111111111111111111111111");
    }
    pub mod slot_history {
        bovey_pubkey::declare_id!("SysvarS1otHistory11111111111111111111111111");
    }
    pub mod stake_history {
        bovey_pubkey::declare_id!("SysvarStakeHistory1111111111111111111111111");
    }
}

pub mod zk_token_proof_program {
    bovey_pubkey::declare_id!("ZkTokenProof1111111111111111111111111111111");
}

pub mod zk_elgamal_proof_program {
    bovey_pubkey::declare_id!("ZkE1Gama1Proof11111111111111111111111111111");
}

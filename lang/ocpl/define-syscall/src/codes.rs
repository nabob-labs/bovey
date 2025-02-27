//! These are syscall codes specified in SIMD-0178.
//! If a new syscall is to be included, add a new number constant
//! for correct registration.

macro_rules! define_code {
    ($name:ident, $code:literal) => {
        pub const $name: u32 = $code;
    };
}

define_code!(ABORT, 1);
define_code!(BOV_PANIC, 2);
define_code!(BOV_MEMCPY_, 3);
define_code!(BOV_MEMMOVE_, 4);
define_code!(BOV_MEMSET_, 5);
define_code!(BOV_MEMCMP_, 6);
define_code!(BOV_LOG_, 7);
define_code!(BOV_LOG_64_, 8);
define_code!(BOV_LOG_PUBKEY, 9);
define_code!(BOV_LOG_COMPUTE_UNITS_, 10);
define_code!(BOV_ALLOC_FREE_, 11);
define_code!(BOV_INVOKE_SIGNED_C, 12);
define_code!(BOV_INVOKE_SIGNED_RUST, 13);
define_code!(BOV_SET_RETURN_DATA, 14);
define_code!(BOV_GET_RETURN_DATA, 15);
define_code!(BOV_LOG_DATA, 16);
define_code!(BOV_SHA256, 17);
define_code!(BOV_KECCAK256, 18);
define_code!(BOV_SECP256K1_RECOVER, 19);
define_code!(BOV_BLAKE3, 20);
define_code!(BOV_POSEIDON, 21);
define_code!(BOV_GET_PROCESSED_SIBLING_INSTRUCTION, 22);
define_code!(BOV_GET_STACK_HEIGHT, 23);
define_code!(BOV_CURVE_VALIDATE_POINT, 24);
define_code!(BOV_CURVE_GROUP_OP, 25);
define_code!(BOV_CURVE_MULTISCALAR_MUL, 26);
define_code!(BOV_CURVE_PAIRING_MAP, 27);
define_code!(BOV_ALT_BN128_GROUP_OP, 28);
define_code!(BOV_ALT_BN128_COMPRESSION, 29);
define_code!(BOV_BIG_MOD_EXP, 30);
define_code!(BOV_REMAINING_COMPUTE_UNITS, 31);
define_code!(BOV_CREATE_PROGRAM_ADDRESS, 32);
define_code!(BOV_TRY_FIND_PROGRAM_ADDRESS, 33);
define_code!(BOV_GET_SYSVAR, 34);
define_code!(BOV_GET_EPOCH_STAKE, 35);
define_code!(BOV_GET_CLOCK_SYSVAR, 36);
define_code!(BOV_GET_EPOCH_SCHEDULE_SYSVAR, 37);
define_code!(BOV_GET_LAST_RESTART_SLOT, 38);
define_code!(BOV_GET_EPOCH_REWARDS_SYSVAR, 39);
define_code!(BOV_GET_FEES_SYSVAR, 40);
define_code!(BOV_GET_RENT_SYSVAR, 41);

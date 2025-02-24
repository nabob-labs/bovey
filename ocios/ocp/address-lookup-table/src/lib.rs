#![allow(incomplete_features)]
#![cfg_attr(feature = "frozen-abi", feature(specialization))]

#[cfg(not(target_os = "bovey"))]
pub mod processor;

//! Definitions for the native BOV token and its fractional lamports.

#![allow(clippy::arithmetic_side_effects)]

/// There are 10^9 lamports in one BOV
pub const LAMPORTS_PER_BOV: u64 = 1_000_000_000;

/// Approximately convert fractional native tokens (lamports) into native tokens (BOV)
pub fn lamports_to_bov(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_BOV as f64
}

/// Approximately convert native tokens (BOV) into fractional native tokens (lamports)
pub fn bov_to_lamports(bov: f64) -> u64 {
    (bov * LAMPORTS_PER_BOV as f64) as u64
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Bov(pub u64);

impl Bov {
    fn write_in_bov(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_BOV,
            self.0 % LAMPORTS_PER_BOV
        )
    }
}

impl Display for Bov {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_bov(f)
    }
}

impl Debug for Bov {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_bov(f)
    }
}

#[deprecated(
    since = "2.1.0",
    note = "Use bovey-clock and bovey-epoch-schedule crates instead."
)]
pub use {
    bovey_clock::{Epoch, Slot, DEFAULT_SLOTS_PER_EPOCH},
    bovey_epoch_schedule::*,
};

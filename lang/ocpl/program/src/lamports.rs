//! Re-exports the [`LamportsError`] type for backwards compatibility.
#[deprecated(
    since = "2.1.0",
    note = "Use bovey_instruction::error::LamportsError instead"
)]
pub use bovey_instruction::error::LamportsError;

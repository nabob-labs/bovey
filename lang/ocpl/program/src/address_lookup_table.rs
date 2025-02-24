#[deprecated(
    since = "2.2.0",
    note = "Use bovey-address-lookup-table interface instead"
)]
pub use bovey_address_lookup_table_interface::{error, instruction, program, state};
pub use bovey_message::AddressLookupTableAccount;

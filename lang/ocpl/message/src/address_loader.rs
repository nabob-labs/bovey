use crate::v0::{LoadedAddresses, MessageAddressTableLookup};
#[deprecated(
    since = "2.1.0",
    note = "Use bovey_transaction_error::AddressLoaderError instead"
)]
pub use bovey_transaction_error::AddressLoaderError;

pub trait AddressLoader: Clone {
    fn load_addresses(
        self,
        lookups: &[MessageAddressTableLookup],
    ) -> Result<LoadedAddresses, AddressLoaderError>;
}

#[derive(Clone)]
pub enum SimpleAddressLoader {
    Disabled,
    Enabled(LoadedAddresses),
}

impl AddressLoader for SimpleAddressLoader {
    fn load_addresses(
        self,
        _lookups: &[MessageAddressTableLookup],
    ) -> Result<LoadedAddresses, AddressLoaderError> {
        match self {
            Self::Disabled => Err(AddressLoaderError::Disabled),
            Self::Enabled(loaded_addresses) => Ok(loaded_addresses),
        }
    }
}

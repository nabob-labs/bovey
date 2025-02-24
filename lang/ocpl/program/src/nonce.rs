pub use bovey_nonce::{state::State, NONCED_TX_MARKER_IX_INDEX};
pub mod state {
    pub use bovey_nonce::{
        state::{Data, DurableNonce, State},
        versions::{AuthorizeNonceError, Versions},
    };
}

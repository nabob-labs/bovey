#![allow(clippy::arithmetic_side_effects)]

pub mod connection_cache;
pub mod nonblocking;
pub mod send_and_confirm_transactions_in_parallel;
pub mod thin_client;
pub mod tpu_client;
pub mod transaction_executor;

pub use bovey_rpc_client::mock_sender_for_cli;

pub mod blockhash_query {
    pub use bovey_rpc_client_nonce_utils::blockhash_query::*;
}
pub mod client_error {
    pub use bovey_rpc_client_api::client_error::{
        reqwest, Error as ClientError, ErrorKind as ClientErrorKind, Result,
    };
}
/// Durable transaction nonce helpers.
pub mod nonce_utils {
    pub use bovey_rpc_client_nonce_utils::*;
}
pub mod pubsub_client {
    pub use bovey_pubsub_client::pubsub_client::*;
}
/// Communication with a Bovey node over RPC.
///
/// Software that interacts with the Bovey blockchain, whether querying its
/// state or submitting transactions, communicates with a Bovey node over
/// [JSON-RPC], using the [`RpcClient`] type.
///
/// [JSON-RPC]: https://www.jsonrpc.org/specification
/// [`RpcClient`]: crate::rpc_client::RpcClient
pub mod rpc_client {
    pub use bovey_rpc_client::rpc_client::*;
}
pub mod rpc_config {
    pub use bovey_rpc_client_api::config::*;
}
/// Implementation defined RPC server errors
pub mod rpc_custom_error {
    pub use bovey_rpc_client_api::custom_error::*;
}
pub mod rpc_filter {
    pub use bovey_rpc_client_api::filter::*;
}
pub mod rpc_request {
    pub use bovey_rpc_client_api::request::*;
}
pub mod rpc_response {
    pub use bovey_rpc_client_api::response::*;
}
/// A transport for RPC calls.
pub mod rpc_sender {
    pub use bovey_rpc_client::rpc_sender::*;
}

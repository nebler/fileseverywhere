use std::error::Error;

use async_trait::async_trait;

// Peer represents the remote node
pub trait Peer {}

// Transport is anything that handles the communicaiton between the nodes
// in the network. This can be of the form (TCP, UDP, Websockets, ...)
#[async_trait]
pub trait Transport: Send + Sync {
    async fn start_accept(&self) -> Result<(), Box<dyn Error>>;
}

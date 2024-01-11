use std::io;

use async_trait::async_trait;

#[async_trait]
pub trait Decoder {
    fn decode(&self, reader: &dyn tokio::io::AsyncRead);
}

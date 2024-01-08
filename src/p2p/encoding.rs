use std::io;

use async_trait::async_trait;

#[async_trait]
pub trait Decoder {
    fn decode<T>(&self, reader: &mut dyn io::Read) -> Result<T, io::Error>;
}

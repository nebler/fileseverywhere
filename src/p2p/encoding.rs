use std::io;

use async_trait::async_trait;

pub trait Decoder: Send + Sync {
    fn test(&self) {
        println!("decoding!!!")
    }
}

use std::arch::aarch64::int32x2_t;

use tokio::{io::AsyncRead, net::TcpStream};

// pub trait Decoder {
//     type Fun: Fn(dyn AsyncRead) -> String;
//     fn decoding_funciton(&self) -> Self::Fun;
// }

use tokio::net::TcpStream as TokioTcpStream;

pub struct DefaultDecoder {}

impl DefaultDecoder {
    pub fn decoding_function(&self) -> Box<dyn Fn(TokioTcpStream) -> String + Send + Sync> {
        Box::new(default_encoding_function)
    }
}

fn default_encoding_function(read: TokioTcpStream) -> String {
    println!("We are encoding {:?}", read);
    return "hello".to_string();
}

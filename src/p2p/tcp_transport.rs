use async_trait::async_trait;
use std::io::{Read, Write};
use std::str::from_utf8;
use tokio::net::{TcpListener, TcpStream};

use super::decoder::DefaultDecoder;

pub struct TcpPeer {
    conn: TcpStream,
    outbound: bool,
}

pub struct TCPTransport {
    listener: TcpListener,
}

impl TCPTransport {
    pub async fn start(&self) {
        loop {
            for mut connection in self.listener.accept().await {
                tokio::spawn(async move {
                    let decode = DefaultDecoder::decode(&mut connection.0).await;
                    let rpc = decode.expect("oh no");
                    println!("we are done encoding");
                    println!(
                        "this is the rpc: from {} stream: {} payload: {:?}",
                        rpc.from,
                        rpc.stream,
                        from_utf8(&rpc.payload)
                    );
                    println!("test")
                });
            }
        }
    }
    pub async fn new_tcp_transport(address: &str) -> Self {
        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to bind to address");
        let decoder = DefaultDecoder {};
        TCPTransport { listener }
    }
}

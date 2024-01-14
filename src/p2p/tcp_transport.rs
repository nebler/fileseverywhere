use async_trait::async_trait;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

use super::decoder::Decoder;
use super::{decoder::DefaultDecoder, message::RPC, transport::Transport};

pub struct TcpPeer {
    conn: TcpStream,
    outbound: bool,
}

pub struct TCPTransport {
    listener: TcpListener,
    decoder: DefaultDecoder,
}

impl Transport for TCPTransport {
    fn start(&self) {
        for connection in self.listener.incoming() {
            match connection {
                Ok(mut stream) => {
                    let decode = self.decoder.decode(&mut stream);
                    let rpc = decode.expect("oh no");
                    println!("we are done encoding");
                    println!(
                        "this is the rpc: from {} stream: {} payload: {:?}",
                        rpc.from,
                        rpc.stream,
                        from_utf8(&rpc.payload)
                    )
                }
                Err(e) => {
                    // Handle error, e.g., print or log it
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

impl TCPTransport {
    pub fn new_tcp_transport(address: &str) -> Self {
        let listener = TcpListener::bind(address).expect("Failed to bind to address");
        let decoder = DefaultDecoder {};
        TCPTransport { listener, decoder }
    }
}

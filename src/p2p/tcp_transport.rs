use async_trait::async_trait;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use super::decoder::Decoder;
use super::message;
use super::{decoder::DefaultDecoder, message::RPC, transport::Transport};

pub struct TcpPeer {
    conn: TcpStream,
    outbound: bool,
}

pub struct TCPTransport {
    listener: TcpListener,
    decoder: DefaultDecoder,
}

pub fn new_tcp_transport(address: &str) -> TCPTransport {
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    let decoder = DefaultDecoder {};
    TCPTransport { listener, decoder }
}

impl Transport for TCPTransport {
    fn start(&self) {
        for connection in self.listener.incoming() {
            match connection {
                Ok(stream) => {
                    let message = RPC {
                        from: todo!(),
                        payload: todo!(),
                        stream: todo!(),
                    };
                    let decode = self.decoder.decode(&mut connection.unwrap(), &mut message);
                }
                Err(e) => {
                    // Handle error, e.g., print or log it
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

use async_trait::async_trait;
use tokio::net::{TcpListener, TcpStream};

use super::{
    decoder::DefaultDecoder,
    transport::{self, Transport},
};

pub struct TcpPeer {
    // conn is the underlying connection of the peer
    conn: TcpStream,
    // if we dial and retieve a stream => outbound == true
    // if we accept and retieve a stream => outbound == false
    outbound: bool,
}

pub struct TCPTransport {
    listener_address: String,
    listener: TcpListener,
    decoder: DefaultDecoder,
}

pub async fn new_tcp_transport(address: &str) -> TCPTransport {
    let listener = TcpListener::bind(address).await.unwrap();
    let mut addr: String = address.to_owned();
    let decoder = DefaultDecoder {};
    return TCPTransport {
        listener_address: addr,
        listener: listener,
        decoder: decoder,
    };
}
#[async_trait(?Send)]
impl Transport for TCPTransport {
    async fn start(&self) {
        loop {
            // The second item contains the IP and port of the new connection.
            let (connection, _) = self.listener.accept().await.unwrap();
            let decode = self.decoder.decoding_function();
            tokio::spawn(async move {
                process(connection, decode).await;
            });
        }
    }
}

async fn process(connection: TcpStream, decode: Box<dyn Fn(TcpStream) -> String + Send + Sync>) {
    decode(connection);
}

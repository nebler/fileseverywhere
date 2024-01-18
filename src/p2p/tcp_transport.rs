use async_trait::async_trait;
use std::io::{Error, Read, Write};
use std::str::from_utf8;
use tokio::net::{TcpListener, TcpStream};

use super::decoder::DefaultDecoder;

pub struct TcpPeer {
    conn: TcpStream,
    outbound: bool,
}

pub struct TCPTransport {}

impl TCPTransport {
    pub async fn start(&self, listener: TcpListener) {
        loop {
            println!("heyyyy");
            for connection in listener.accept().await {
                Self::handle_connection(connection.0).await;
            }
        }
    }

    async fn handle_connection(mut connection: TcpStream) {
        tokio::spawn(async move {
            let decode = DefaultDecoder::decode(&mut connection).await;
            let rpc = decode.expect("oh no");
            println!("we are done encoding");
            println!(
                "this is the rpc: from {} stream: {} payload: {:?}",
                rpc.from,
                rpc.stream,
                from_utf8(&rpc.payload)
            );
            println!("test5")
        });
    }
    pub async fn tcp_transport(address: &str) -> Result<(), Error> {
        let tcp_transport = TCPTransport {};
        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to bind to address");
        tcp_transport.start(listener).await;
        Ok(())
    }

    pub async fn dial(address: &str) -> Result<(), Error> {
        let connection = TcpStream::connect(&address).await?;
        Self::handle_connection(connection).await;
        Ok(())
    }
}

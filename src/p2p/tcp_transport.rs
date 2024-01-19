use async_trait::async_trait;
use std::io::{Error, Read, Write};
use std::str::from_utf8;
use tokio::io::AsyncWriteExt;
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
    async fn start(&self) {
        loop {
            println!("heyyyy");
            for connection in self.listener.accept().await {
                Self::handle_connection(connection.0, false).await;
            }
        }
    }

    async fn handle_connection(mut connection: TcpStream, outbound: bool) -> Result<(), Error> {
        tokio::spawn(async move {
            let decoded = DefaultDecoder::decode(&mut connection).await;
            let peer = TcpPeer {
                conn: connection,
                outbound: outbound,
            };
            let rpc = decoded.expect("oh no");
            println!("we are done encoding");
            println!(
                "this is the rpc: from {} stream: {} payload: {:?}",
                rpc.from,
                rpc.stream,
                from_utf8(&rpc.payload)
            );

            if (rpc.stream) {
                println!("hellooo")
            }
            println!("test5");
        });
        Ok(())
    }
    pub async fn tcp_transport(address: &str) -> TCPTransport {
        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to bind to address");
        let tcp_transport = TCPTransport { listener };
        tcp_transport
    }

    pub async fn close(&self) {
        //is this good practice
        self.listener.shutdown();
    }

    pub async fn dial(&self, address: &str) -> Result<(), Error> {
        let connection = TcpStream::connect(&address).await?;
        Self::handle_connection(connection, true).await;
        Ok(())
    }
}

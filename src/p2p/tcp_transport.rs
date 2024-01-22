use std::{fmt::Error, str::from_utf8};

use tokio::net::{TcpListener, TcpStream};

use crate::p2p::decoder::DefaultDecoder;

pub struct TcpPeer {
    conn: TcpStream,
    outbound: bool,
}

pub struct TCPTransport {
    listener: TcpListener,
    peers: Vec<TcpPeer>,
}

impl TCPTransport {
    async fn start(&self) {
        loop {
            println!("heyyyy");
            for connection in self.listener.accept().await {
                self.handle_connection(connection.0, false).await;
            }
        }
    }

    async fn handle_connection(
        &self,
        mut connection: TcpStream,
        outbound: bool,
    ) -> Result<(), Error> {
        tokio::spawn(async move {
            let decoded = DefaultDecoder::decode(&mut connection).await;
            let rpc = decoded.expect("oh no");
            println!("we are done encoding");
            println!(
                "this is the rpc: from {} stream: {} payload: {:?}",
                rpc.from,
                rpc.stream,
                from_utf8(&rpc.payload)
            );

            if (rpc.stream) {
                println!("we are streaming boys")
            }
            let peer = TcpPeer {
                conn: connection,
                outbound: outbound,
            };

            // todo: I need a way to access self.peers from this thread now so i can put the peers inside there without the compiler
            // screaming at me
        });
        return Ok(());
    }
    pub async fn tcp_transport(address: &str) -> TCPTransport {
        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to bind to address");
        let tcp_transport = TCPTransport {
            listener,
            peers: Vec::new(),
        };
        tcp_transport
    }

    pub async fn close(&self) {
        //is this good practice?
    }

    pub async fn dial(&self, address: &str) -> Result<(), Error> {
        let connection = TcpStream::connect(&address).await?;
        self.handle_connection(connection, true).await;
        Ok(())
    }
}

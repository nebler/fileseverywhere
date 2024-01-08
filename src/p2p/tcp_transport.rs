use std::{collections::HashMap, error::Error, io, result::Result, sync::Mutex};

use async_trait::async_trait;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use super::{
    encoding::Decoder,
    handshaker::{new_default_handshaker, DefaultHandshaker},
    transport::{Peer, Transport},
};

pub struct TcpTransport {
    listener_adress: String,
    listener: TcpListener,
    mu: Option<Mutex<TcpPeers>>,
    peers: Option<TcpPeers>,
    decoder: Option<Box<dyn Decoder + Send + Sync>>,
    handshaker: DefaultHandshaker,
}

pub struct TcpPeer {
    // conn is the underlying connection of the peer
    conn: TcpStream,
    // if we dial and retieve a stream => outbound == true
    // if we accept and retieve a stream => outbound == false
    outbound: bool,
}

pub fn new_tcp_peer(outbound: bool, conn: TcpStream) -> TcpPeer {
    return TcpPeer {
        conn: conn,
        outbound: outbound,
    };
}
// TcpPeers represents the remote node over a TCP established connection
struct TcpPeers(HashMap<String, Box<dyn Peer + Send + Sync>>);

pub async fn new_tcp_transport(listener_adress: &str) -> TcpTransport {
    let mut addr: String = "127.0.0.1:".to_owned();
    addr.push_str(listener_adress);
    return TcpTransport {
        listener_adress: listener_adress.to_string(),
        listener: TcpListener::bind(addr).await.unwrap(),
        mu: None,
        peers: None,
        handshaker: new_default_handshaker(),
        decoder: None,
    };
}

#[async_trait]
impl Transport for TcpTransport {
    async fn start_accept(&self) -> Result<(), Box<dyn Error>> {
        println!("start and accept");
        loop {
            let (connection, _) = self.listener.accept().await?;
            self.hand_shake(&connection);
            self.handle_client(connection).await;
            // let status_line = "HTTP/1.1 200 OK";
            // let contents = "Hello World";
            // let length = contents.len();
            // let response =
            //     format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            // println!("{}", response);
            // socket.write_all(response.as_bytes()).await.unwrap();
        }
    }
}

impl TcpTransport {
    fn hand_shake(&self, stream: &TcpStream) {
        self.handshaker.hand_shake(stream)
    }

    async fn handle_client(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        stream.readable().await;
        let mut buffer = [0; 1024];
        tokio::spawn(async move {
            // Wait for the socket to be readable
            // Creating the buffer **after** the `await` prevents it from
            // being stored in the async task.
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            loop {
                self.decoder
                    .expect("decoder isnt set")
                    .decode(stream)
                    .expect("error during decoding");
                while let Ok(bytes_read) = stream.read(&mut buffer).await {
                    if bytes_read == 0 {
                        // Connection closed by the remote peer
                        break;
                    }

                    // Convert received bytes to a UTF-8 string
                    if let Ok(utf8_string) = std::str::from_utf8(&buffer[..bytes_read]) {
                        // Process the received string (e.g., print it)
                        println!("Received: {}", utf8_string);
                    } else {
                        // Invalid UTF-8 sequence, handle or log the error
                        println!("Received invalid UTF-8 sequence");
                    }
                }
            }
        });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn my_test() {
        let listener_addr = "8080";
        let tcp_listener = new_tcp_transport(listener_addr).await;
        assert_eq!(tcp_listener.listener_adress, listener_addr)
    }
}

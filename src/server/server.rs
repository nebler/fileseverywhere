use crate::p2p::tcp_transport::TCPTransport;

pub struct Server {
    tcp_transport: TCPTransport,
}

pub async fn make_server(address: &str) -> Server {
    let tcp_transport = TCPTransport::tcp_transport(address).await;
    return Server {
        tcp_transport: tcp_transport,
    };
}

use p2p::tcp_transport::TCPTransport;

mod p2p;
#[tokio::main]
async fn main() {
    let tcp = TCPTransport::new_tcp_transport("127.0.0.1:3000").await;
    let _ = tcp.start().await;
}

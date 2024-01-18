use futures::{future::TryFutureExt, try_join};
use p2p::tcp_transport::TCPTransport;
mod p2p;
#[tokio::main]
async fn main() {
    let tcp = TCPTransport::tcp_transport("127.0.0.1:3000");
    let tcp2 = TCPTransport::tcp_transport("127.0.0.1:3001");
    let tcp3 = TCPTransport::tcp_transport("127.0.0.1:3002");
    if let Err(err) = tokio::try_join!(tcp, tcp2, tcp3) {
        eprintln!("Error: {}", err);
    }
    println!("hello")
}

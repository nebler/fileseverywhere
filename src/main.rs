use p2p::{tcp_transport::TCPTransport, transport::Transport};

mod p2p;
fn main() {
    let tcp = TCPTransport::new_tcp_transport("127.0.0.1:3000");
    let _ = tcp.start();
}

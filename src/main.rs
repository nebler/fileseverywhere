mod p2p;
use p2p::{tcp_transport::new_tcp_transport, transport::Transport};
fn main() {
    let tcp = new_tcp_transport("127.0.0.1:3000");
    let _ = tcp.start();
}

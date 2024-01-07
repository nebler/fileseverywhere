mod p2p;
use crate::p2p::{tcp_transport::*, transport::Transport};
async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    let test_tcp = new_tcp_transport("3000").await;
    let err = test_tcp.start_accept().await;

    // This println! comes first
    println!("hello");
}

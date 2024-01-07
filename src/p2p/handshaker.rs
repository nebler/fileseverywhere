use tokio::net::TcpStream;

pub struct DefaultHandshaker {}

impl DefaultHandshaker {
    pub fn hand_shake(&self, connection: &TcpStream) {
        println!("we just shaking some hands here {:?}", connection)
    }
}

pub fn new_default_handshaker() -> DefaultHandshaker {
    return DefaultHandshaker {};
}

pub fn no_op_handshake() {}

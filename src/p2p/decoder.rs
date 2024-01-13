use std::{
    io::{self, Read},
    net::TcpStream,
};

use super::message::{INCOMING_STREAM, RPC};

pub trait Decoder {
    fn decode(&self, reader: &mut TcpStream, msg: &mut RPC) -> io::Result<()>;
}

pub struct DefaultDecoder;

impl Decoder for DefaultDecoder {
    fn decode(&self, reader: &mut TcpStream, msg: &mut RPC) -> io::Result<()> {
        let mut peek_buf = [0; 1];
        let from = reader.peer_addr().expect("oh wow");
        if reader.read_exact(&mut peek_buf).is_err() {
            return Ok(());
        }
        // In case of a stream, we are not decoding what is being sent over the network.
        // We are just setting Stream true so we can handle that in our logic.
        let stream = peek_buf[0] == INCOMING_STREAM;
        if stream {
            msg.stream = true;
            return Ok(());
        }

        let mut buf = vec![0; 1028];
        let n = reader.read(&mut buf)?;

        msg.payload = buf[..n].to_vec();
        Ok(())
    }
}

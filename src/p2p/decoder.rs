use std::{
    io::{self, Read},
    net::TcpStream,
};

use super::message::{INCOMING_STREAM, RPC};

pub trait Decoder {
    fn decode(&self, reader: &mut TcpStream) -> Result<RPC, io::Error>;
}

pub struct DefaultDecoder;

impl Decoder for DefaultDecoder {
    fn decode(&self, reader: &mut TcpStream) -> Result<RPC, io::Error> {
        let mut peek_buf = [0; 1];

        println!("we are encoding");
        let from = reader.peer_addr().expect("oh wow");
        if reader.read_exact(&mut peek_buf).is_err() {
            return Ok(RPC {
                from: from.to_string(),
                payload: [].to_vec(),
                stream: false,
            });
        }
        // In case of a stream, we are not decoding what is being sent over the network.
        // We are just setting Stream true so we can handle that in our logic.
        let stream = peek_buf[0] == INCOMING_STREAM;
        if stream {
            return Ok(RPC {
                from: from.to_string(),
                payload: [].to_vec(),
                stream: true,
            });
        }

        let mut buf = vec![0; 1028];
        let n = reader.read(&mut buf)?;
        Ok(RPC {
            from: from.to_string(),
            payload: buf[..n].to_vec(),
            stream: false,
        })
    }
}

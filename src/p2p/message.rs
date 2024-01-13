pub const INCOMING_MESSAGE: u8 = 0x1;
pub const INCOMING_STREAM: u8 = 0x2;

#[derive(Debug)]
pub struct RPC {
    pub from: String,
    pub payload: Vec<u8>,
    pub stream: bool,
}

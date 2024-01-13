use async_trait::async_trait;

pub trait Peer {}
pub trait Transport {
    fn start(&self);
}

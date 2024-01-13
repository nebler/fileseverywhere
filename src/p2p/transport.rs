use async_trait::async_trait;

pub trait Peer {}

#[async_trait(?Send)]
pub trait Transport {
    async fn start(&self);
}

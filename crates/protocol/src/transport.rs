use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait TransportReader {
    /// Receive next raw frame or None on closed
    async fn recv(&mut self) -> Option<Result<Bytes>>;
}

#[async_trait]
pub trait TransportWriter {
    /// Send a raw frame of bytes
    async fn send(&mut self, data: Bytes) -> Result<()>;
}

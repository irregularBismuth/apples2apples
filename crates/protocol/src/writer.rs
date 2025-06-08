use anyhow::Result;
use bytes::Bytes;
use futures::SinkExt;
use tokio::io::AsyncWrite;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

pub struct MessageWriter<W>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    inner: FramedWrite<W, LengthDelimitedCodec>,
}

impl<W> MessageWriter<W>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(write: W) -> Self {
        MessageWriter {
            inner: FramedWrite::new(write, LengthDelimitedCodec::new()),
        }
    }

    /// Send one raw frame of bytes
    async fn send_bytes(&mut self, data: Bytes) -> Result<()> {
        self.inner.send(data).await.map_err(Into::into)
    }

    /// Serialize `msg` to JSON and send it.
    pub async fn send_message<M: serde::Serialize>(&mut self, msg: &M) -> Result<()> {
        let buf = serde_json::to_vec(msg)?;
        self.send_bytes(Bytes::from(buf)).await
    }
}

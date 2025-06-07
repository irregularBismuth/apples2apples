use crate::transport::Transport;
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use futures::SinkExt;
use futures::StreamExt;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub struct FramedTransport<R, W>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    reader: FramedRead<R, LengthDelimitedCodec>,
    writer: FramedWrite<W, LengthDelimitedCodec>,
}

impl<R, W> FramedTransport<R, W>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    /// Build a new framed transport over any AsyncRead/AsyncWrite pair
    pub fn new(read: R, write: W) -> Self {
        let codec = LengthDelimitedCodec::new();
        FramedTransport {
            reader: FramedRead::new(read, codec.clone()),
            writer: FramedWrite::new(write, codec),
        }
    }
}

#[async_trait]
impl<R, W> Transport for FramedTransport<R, W>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    async fn send(&mut self, data: Bytes) -> Result<()> {
        self.writer.send(data).await.map_err(Into::into)
    }

    async fn recv(&mut self) -> Option<Result<Bytes>> {
        match self.reader.next().await {
            Some(Ok(buf)) => Some(Ok(buf.freeze())),
            Some(Err(e)) => Some(Err(e.into())),
            None => None,
        }
    }
}

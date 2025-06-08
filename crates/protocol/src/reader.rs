use anyhow::Result;
use bytes::Bytes;
use futures::StreamExt;
use tokio::io::AsyncRead;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

pub struct MessageReader<R>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    inner: FramedRead<R, LengthDelimitedCodec>,
}

impl<R> MessageReader<R>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    pub fn new(read: R) -> Self {
        MessageReader {
            inner: FramedRead::new(read, LengthDelimitedCodec::new()),
        }
    }

    /// Pull the next raw frame off the wire.
    /// Returns `None` when the stream is closed.
    async fn next_frame(&mut self) -> Option<Result<Bytes>> {
        match self.inner.next().await {
            Some(Ok(buf)) => Some(Ok(buf.freeze())),
            Some(Err(e)) => Some(Err(e.into())),
            None => None,
        }
    }

    /// Convenience: deserialize the next frame from JSON into `M`.
    pub async fn next_message<M: serde::de::DeserializeOwned>(&mut self) -> Option<Result<M>> {
        self.next_frame()
            .await
            .map(|res| res.and_then(|bytes| serde_json::from_slice(&bytes).map_err(Into::into)))
    }
}

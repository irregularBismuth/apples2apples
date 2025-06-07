use {
    crate::transport::Transport,
    anyhow::Result,
    bytes::Bytes,
    serde::{de::DeserializeOwned, Serialize},
};

pub struct Protocol<T: Transport> {
    transport: T,
}

impl<T: Transport> Protocol<T> {
    pub fn new(transport: T) -> Self {
        Protocol { transport }
    }
    ///Send the message over the transport
    pub async fn send_message<M: Serialize>(&mut self, msg: &M) -> Result<()> {
        let buf = serde_json::to_vec(&msg)?;
        self.transport.send(Bytes::from(buf)).await
    }
    /// Return the next message from the transport
    pub async fn next_message<M: DeserializeOwned>(&mut self) -> Option<Result<M>> {
        match self.transport.recv().await {
            Some(Ok(buf)) => Some(serde_json::from_slice(&buf).map_err(Into::into)),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

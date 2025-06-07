use crate::framed_transport::FramedTransport;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
pub type TcpTransport = FramedTransport<OwnedReadHalf, OwnedWriteHalf>;

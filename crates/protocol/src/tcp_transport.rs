use {
    crate::framed_transport::FramedTransport,
    tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub type TcpTransport = FramedTransport<OwnedReadHalf, OwnedWriteHalf>;

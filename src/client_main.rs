use crate::actors::client_fsm::ClientFsm;
use anyhow::Result;
use apples_protocol::framed_transport::FramedTransport;
use apples_protocol::protocol::Protocol;
use std::net::SocketAddrV4;
use tokio::net::TcpStream;
#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    let stream = TcpStream::connect(socket).await?;
    let (read, write) = stream.into_split();
    let transport = FramedTransport::new(read, write);
    let mut protocol = Protocol::new(transport);
    let _ = protocol
        .send_message(&apples_core::protocol::message::GameMessage::GameEnd)
        .await?;

    let (client, handle) = ractor::Actor::spawn(None, ClientFsm, ()).await?;
    println!("{}", socket.ip());
    println!("{}", socket.port());
    handle.await?;
    Ok(())
}

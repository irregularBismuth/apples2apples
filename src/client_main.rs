use crate::actors::client_fsm::ClientFsm;
use crate::actors::networking::connection::Connection;
use anyhow::Result;
use apples_protocol::framed_transport::FramedTransport;
use apples_protocol::reader::Protocol;
use std::net::SocketAddrV4;
use tokio::net::TcpStream;
#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    let stream = TcpStream::connect(socket).await?;
    let (connector, _) = ractor::Actor::spawn(None, Connection, stream).await?;
    //   let (client, handle) = ractor::Actor::spawn(None, ClientFsm, ()).await?;
    ractor::cast!(
        connector,
        crate::actors::networking::connection::ConnectionMsg::Send(
            apples_core::protocol::message::GameMessage::GameEnd
        )
    )?;
    println!("{}", socket.ip());
    println!("{}", socket.port());
    loop {}
    //  handle.await?;
    Ok(())
}

use crate::actors::client_fsm::ClientFsm;
use crate::actors::networking::connection::Connection;
use crate::actors::networking::registry::ConnectionRegistry;
use anyhow::Result;
use std::net::SocketAddrV4;
use tokio::net::TcpStream;
#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    let stream = TcpStream::connect(socket).await?;
    let (connector, _) = ractor::Actor::spawn(None, Connection, (stream, None)).await?;
    let (client, handle) = ractor::Actor::spawn(None, ClientFsm, ()).await?;
    handle.await?;
    Ok(())
}

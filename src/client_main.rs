use crate::actors::client_fsm::ClientFsm;
use crate::actors::networking::connection::Connection;
use crate::actors::networking::registry::{ConnectionRegistry, RegistryType};
use anyhow::Result;
use std::net::SocketAddrV4;
use tokio::net::TcpStream;
#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    let stream = TcpStream::connect(socket).await?;
    let (connector, _) = ractor::Actor::spawn(None, Connection, stream).await?;

    let (registry, _) =
        ractor::Actor::spawn(None, ConnectionRegistry, RegistryType::Client).await?;

    ractor::cast!(
        registry,
        crate::actors::networking::registry::RegistryMsg::AddClient(0, connector)
    )?;

    ractor::cast!(
        registry,
        crate::actors::networking::registry::RegistryMsg::Unicast(
            0,
            apples_core::protocol::message::GameMessage::GameEnd
        )
    )?;
    //let (client, handle) = ractor::Actor::spawn(None, ClientFsm, ()).await?;

    println!("{}", socket.ip());
    println!("{}", socket.port());
    //handle.await?;
    loop {}
    Ok(())
}

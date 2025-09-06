use crate::actors::client_fsm::ClientFsm;
use crate::actors::networking::connection::Connection;
use crate::actors::networking::connection::InboundTarget;
use crate::actors::networking::registry::ConnectionRegistry;
use crate::actors::players::human::HumanPlayer;
use anyhow::Result;
use std::net::SocketAddrV4;
use tokio::net::TcpStream;

#[doc = "client.md"]
pub async fn client_main(socket: SocketAddrV4) -> Result<()> {
    let stream = TcpStream::connect(socket).await?;

    let (client, handle) = ractor::Actor::spawn(None, ClientFsm, ()).await?;

    let (connector, _) = ractor::Actor::spawn(
        None,
        Connection,
        (stream, InboundTarget::Client { fsm: client }),
    )
    .await?;

    let (player, _) = ractor::Actor::spawn(None, HumanPlayer, ()).await?;

    ractor::cast!(
        player,
        crate::actors::players::human::HumanPlayerMsg::DealCard(
            apples_core::cards::red_card::RedCard::new("test".to_string(), "test".to_string(), 1)
        )
    )?;

    ractor::cast!(
        connector,
        crate::actors::networking::connection::ConnectionMsg::Send(
            apples_core::protocol::message::GameMessage::GameEnd
        )
    )?;

    handle.await?;
    Ok(())
}

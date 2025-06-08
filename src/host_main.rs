use crate::actors::host_fsm::HostFsm;
use crate::actors::networking::{acceptor::Acceptor, registry::ConnectionRegistry};
use crate::deck_handler::DeckHandler;
use anyhow::Result;
use apples_utils::{config::Config, consts::CONFIG_TOML, game_mode::GameMode};
#[doc = include_str!("../doc/host.md")]
pub async fn host_main(players: usize, bots: usize) -> Result<()> {
    let config = Config::parse_config(CONFIG_TOML.into())?;
    match config.game_mode() {
        GameMode::Original => {
            let tcp_listener = tokio::net::TcpListener::bind(config.socket()).await?;
            let (registry, _) = ractor::Actor::spawn(None, ConnectionRegistry, ()).await?;
            let (acceptor, _) =
                ractor::Actor::spawn(None, Acceptor, (tcp_listener, registry)).await?;
            let (fsm, handle) = ractor::Actor::spawn(None, HostFsm, ()).await?;
            let socket = config.socket().clone();
            //TODO: for the bots could be done like this instead thus every player is "remote "and
            //stored in registry uneccesary but works and I think is cleanest approach
            for i in 0..100 {
                tokio::spawn(async move {
                    let stream = tokio::net::TcpStream::connect(socket).await.unwrap();
                    let (connector, _) = ractor::Actor::spawn(
                        None,
                        crate::actors::networking::connection::Connection,
                        stream,
                    )
                    .await
                    .unwrap();
                });
            }
            let _ = handle.await;
        }
        _ => {
            todo!("unsupported now, original is supported")
        }
    }
    Ok(())
}

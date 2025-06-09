use crate::actors::dealer::DealerActor;
use crate::actors::host_fsm::HostFsm;
use crate::actors::host_fsm::HostState;
use crate::actors::networking::{acceptor::Acceptor, registry::ConnectionRegistry};
use crate::actors::player_manager::PlayerManager;
use crate::actors::score_manager::ScoreManager;
use crate::deck_handler::DeckHandler;
use anyhow::Result;
use apples_utils::{config::Config, consts::CONFIG_TOML, game_mode::GameMode};

#[doc = include_str!("../doc/host.md")]
pub async fn host_main(players: usize, bots: usize) -> Result<()> {
    let config = Config::parse_config(CONFIG_TOML.into())?;
    match config.game_mode() {
        GameMode::Original => {
            let tcp_listener = tokio::net::TcpListener::bind(config.socket()).await?;
            let mut deck = DeckHandler::new();
            deck.load_decks(
                config.red_deck_path().into(),
                config.green_deck_path().into(),
            )
            .await?;
            let win_condition = config
                .get_required_apples(players + bots)
                .expect("failed to get win condition");
            deck.shuffle();

            let (player_manager, _) = ractor::Actor::spawn(None, PlayerManager, ()).await?;

            let (score_manager, _) =
                ractor::Actor::spawn(None, ScoreManager, win_condition).await?;

            let (dealer, _) = ractor::Actor::spawn(None, DealerActor, deck).await?;

            let host_state = HostState::new(dealer, score_manager, player_manager);
            let (fsm, handle) = ractor::Actor::spawn(None, HostFsm, host_state).await?;
            let (registry, _) = ractor::Actor::spawn(None, ConnectionRegistry, fsm).await?;
            let (_, _) = ractor::Actor::spawn(None, Acceptor, (tcp_listener, registry)).await?;
            let _ = handle.await;
        }
        _ => {
            todo!("unsupported now, original is supported")
        }
    }
    Ok(())
}

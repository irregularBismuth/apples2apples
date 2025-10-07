use crate::actors::dealer::Dealer;
use crate::actors::score_handler::{Score, ScoreManager, ScoreManagerMsg};
use crate::deck_handler::DeckHandler;
use anyhow::Result;
use apples_utils::{
    config::Config, consts::CONFIG_TOML, game_mode::GameMode, setup_tracing::setup_logging,
};
use core::num::NonZeroUsize;
use ractor::{cast, Actor, ActorProcessingErr, ActorRef};

#[doc = include_str!("../doc/host.md")]
pub async fn host_main(players: usize, bots: usize) -> Result<()> {
    let config = Config::parse_config(CONFIG_TOML.into())?;

    match config.game_mode() {
        GameMode::Original => {
            let _tcp_listener = tokio::net::TcpListener::bind(config.socket()).await?;
            let _win_condition = config
                .get_required_apples(players + bots)
                .expect("failed to get win condition");

            let deck = {
                let mut deck = DeckHandler::new();
                deck.load_decks(
                    config.red_deck_path().into(),
                    config.green_deck_path().into(),
                )
                .await?;
                deck
            };

            let (dealer, _) = Actor::spawn(None, Dealer, deck).await?;
            let (score_manager, _) = Actor::spawn(None, ScoreManager, ()).await?;

            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            let _ = deck;
        }
        _ => todo!("unsupported for now, original is supported"),
    }
    Ok(())
}

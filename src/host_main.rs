use crate::actors::dealer::Dealer;
use crate::actors::score_handler::{Score, ScoreManager, ScoreManagerMsg};
use crate::deck_handler::DeckHandler;
use anyhow::Result;
use apples_utils::{config::Config, consts::CONFIG_TOML, game_mode::GameMode};
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
            let id = apples_core::PlayerId(15);
            let score = Score(154);
            let result = ractor::cast!(score_manager, ScoreManagerMsg::UpdateScore(id, score))?;
            let result = ractor::call!(
                score_manager,
                ScoreManagerMsg::RetrieveScore,
                apples_core::PlayerId(17)
            )?;

            if let Err(e) = result {
                println!("this doesn't exist {}", e);
            } else {
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            let _ = deck;
        }
        _ => todo!("unsupported now, original is supported"),
    }
    Ok(())
}

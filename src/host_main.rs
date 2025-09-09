use crate::actors::dealer::Dealer;
use crate::deck_handler::DeckHandler;
use actor_macros::{actor, actor_handle, actor_pre_start};
use anyhow::Result;
use apples_utils::{config::Config, consts::CONFIG_TOML, game_mode::GameMode};
use core::num::NonZeroUsize;
use ractor::{cast, Actor, ActorProcessingErr, ActorRef};

#[derive(Debug, Clone)]
pub enum PongerMsg {
    Ping(ActorRef<PingerMsg>, u64),
}

#[derive(Debug, Clone)]
pub enum PingerMsg {
    Pong(u64),
}

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
            // let _ = ractor::cast!(dealer, crate::actors::dealer::DealerMsg::Shuffle)?;
            // let amount= NonZeroUsize::new(7000).expect("Failed to construct");
            //  let cards = ractor::call!(dealer, crate::actors::dealer::DealerMsg::DealRedCards,amount)??;

            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

            let _ = deck;
        }
        _ => todo!("unsupported now, original is supported"),
    }
    Ok(())
}

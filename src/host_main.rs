use crate::deck_handler::DeckHandler;
use actor_macros::actor;
use anyhow::Result;
use apples_utils::{config::Config, consts::CONFIG_TOML, game_mode::GameMode};
use ractor::{Actor, ActorProcessingErr, ActorRef, cast};

#[derive(Debug, Clone)]
pub enum PongerMsg {
    Ping(ActorRef<PingerMsg>, u64),
}

#[derive(Debug, Clone)]
pub enum PingerMsg {
    Pong(u64),
}

#[actor(
    msg = PongerMsg,
    state = ()
)]
pub struct Ponger;

impl Ponger {
    pub async fn handle_msg(
        &self,
        _myself: ActorRef<PongerMsg>,
        msg: PongerMsg,
        _state: &mut (),
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            PongerMsg::Ping(reply_to, n) => {
                println!("[Ponger] got Ping({n}), sending Pong({n}) back");
                cast!(reply_to, PingerMsg::Pong(n))
                    .map_err(|e| ActorProcessingErr::from(format!("failed to cast Pong: {e}")))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PingerState {
    remaining: u64,
    ponger: ActorRef<PongerMsg>,
}

#[actor(
    msg = PingerMsg,
    state = PingerState,
    args = (ActorRef<PongerMsg>, u64),
    pre_start = on_start
)]
pub struct Pinger;

impl Pinger {
    pub async fn on_start(
        &self,
        myself: ActorRef<PingerMsg>,
        (ponger, total): (ActorRef<PongerMsg>, u64),
    ) -> Result<PingerState, ActorProcessingErr> {
        println!("[Pinger] starting with {total} exchanges");

        if total > 0 {
            cast!(ponger, PongerMsg::Ping(myself.clone(), 1)).map_err(|e| {
                ActorProcessingErr::from(format!("failed to cast initial Ping: {e}"))
            })?;
        }

        Ok(PingerState {
            remaining: total,
            ponger,
        })
    }

    pub async fn handle_msg(
        &self,
        myself: ActorRef<PingerMsg>,
        msg: PingerMsg,
        state: &mut PingerState,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            PingerMsg::Pong(n) => {
                println!(
                    "[Pinger] got Pong({n}), remaining={}",
                    state.remaining.saturating_sub(1)
                );

                if state.remaining == 0 {
                    return Ok(());
                }

                state.remaining -= 1;

                if state.remaining == 0 {
                    println!("[Pinger] done!");
                } else {
                    let next = n + 1;
                    cast!(state.ponger, PongerMsg::Ping(myself.clone(), next)).map_err(|e| {
                        ActorProcessingErr::from(format!("failed to cast next Ping: {e}"))
                    })?;
                }
            }
        }
        Ok(())
    }
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
                deck.shuffle();
                deck
            };

            let (ponger_ref, _ponger_task) = Actor::spawn(None, Ponger, ()).await?;

            let exchanges: u64 = 5;
            let (_pinger_ref, _pinger_task) =
                Actor::spawn(None, Pinger, (ponger_ref, exchanges)).await?;

            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

            let _ = deck;
        }
        _ => todo!("unsupported now, original is supported"),
    }
    Ok(())
}

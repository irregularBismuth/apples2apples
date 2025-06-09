use crate::actors::players::bot::BotActor;
use crate::actors::players::bot::BotMsg;
use ahash::AHashMap;
use apples_core::player::player::PlayerId;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use ractor_cluster::RactorMessage;

#[derive(RactorMessage)]
pub enum PlayerMsg {
    AddBot(PlayerId),
    AddPlayer(PlayerId),
    GetPlayerAmount(RpcReplyPort<usize>),
}

pub enum PlayerType {
    Bot(ActorRef<BotMsg>),
    Human,
}
pub struct PlayerManager;
pub struct PlayerState {
    players: AHashMap<PlayerId, PlayerType>,
    expected: ExpectedPlayers,
}

pub struct ExpectedHumans(pub usize);
pub struct ExpectedBots(pub usize);
pub struct ExpectedPlayers(pub ExpectedHumans, pub ExpectedBots);

#[ractor::async_trait]
impl Actor for PlayerManager {
    actor_types!(PlayerMsg, PlayerState, ExpectedPlayers);

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(PlayerState {
            players: AHashMap::new(),
            expected: args,
        })
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            PlayerMsg::AddBot(id) => {
                let (player, _) = ractor::Actor::spawn(None, BotActor, id).await?;
            }
            PlayerMsg::AddPlayer(PlayerId(id)) => {
                println!("added player  with id {}", id);
                if state.expected.0 .0 == id {
                    println!("game start we have all players we need {}", id);
                }
            }
            PlayerMsg::GetPlayerAmount(reply) => {
                reply.send(state.players.len())?;
            }
        }
        Ok(())
    }
}

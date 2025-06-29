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
    GetPlayerList(RpcReplyPort<Vec<PlayerId>>),
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
impl ExpectedPlayers {
    ///Return the amount of expected players
    pub fn total(&self) -> usize {
        self.humans() + self.bots()
    }

    ///Return the amount of expected humans
    pub fn humans(&self) -> usize {
        self.0 .0
    }
    ///Return the amount of expected bots
    pub fn bots(&self) -> usize {
        self.1 .0
    }
}

#[ractor::async_trait]
impl Actor for PlayerManager {
    actor_types!(PlayerMsg, PlayerState, ExpectedPlayers);

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        for amount in 0..args.bots() {
            ractor::cast!(myself, PlayerMsg::AddBot(PlayerId(amount)))?;
        }

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
            PlayerMsg::AddPlayer(id) => {
                println!("added player  with id {}", id.0);
                if state.expected.humans() == id.0 {
                    println!("game start we have all players we need {}", id.0);
                } else {
                    state.players.insert(id, PlayerType::Human);
                }
            }
            PlayerMsg::GetPlayerList(reply) => {
                let list: Vec<PlayerId> = state.players.keys().copied().collect();
                reply.send(list)?;
            }
        }
        Ok(())
    }
}

use apples_core::player::player::PlayerId;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use ractor_cluster::RactorMessage;

#[derive(RactorMessage)]
pub enum PlayerMsg {
    AddBot,
    AddPlayer(PlayerId),
    GetPlayerAmount(RpcReplyPort<usize>),
}

pub struct PlayerManager;
pub struct PlayerState {}

#[ractor::async_trait]
impl Actor for PlayerManager {
    actor_types!(PlayerMsg, PlayerState, ());

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(PlayerState {})
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            PlayerMsg::AddBot => {}
            PlayerMsg::AddPlayer(PlayerId(id)) => {
                println!("added player  with id {}", id);
            }
            PlayerMsg::GetPlayerAmount(reply) => {
                reply.send(17)?;
            }
        }
        Ok(())
    }
}

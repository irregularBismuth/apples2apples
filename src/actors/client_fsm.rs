use super::networking::connection::ConnectionMsg;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use ractor_cluster::RactorMessage;

#[derive(RactorMessage)]
pub enum ClientStates {
    AwaitInstruction,
    ReceiveRedCards,
    ChooseRedCard,
    Judge,
    GameOver,
}

pub struct ClientFsm;

#[ractor::async_trait]
impl Actor for ClientFsm {
    actor_types!(ClientStates, (), ());

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ClientStates::AwaitInstruction => {
                ractor::cast!(myself, ClientStates::AwaitInstruction)?;
            }
            ClientStates::Judge => {}
            ClientStates::ChooseRedCard => {}
            ClientStates::ReceiveRedCards => {}
            ClientStates::GameOver => {
                myself.stop(None);
            }
        }
        Ok(())
    }
}

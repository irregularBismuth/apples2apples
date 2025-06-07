use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
pub struct HostFsm;
#[ractor::async_trait]
impl Actor for HostFsm {
    actor_types!((), (), ());

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
        myself.stop(None);
        Ok(())
    }
}

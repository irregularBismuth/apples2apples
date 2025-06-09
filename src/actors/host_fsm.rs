use crate::actors::dealer::DealerTx;
use crate::actors::player_manager::PlayerMsg;
use crate::actors::score_manager::ScoreTx;
use apples_core::player::player::PlayerId;
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use ractor_cluster::RactorMessage;
pub struct HostFsm;

pub struct HostState {
    dealer: DealerTx,
    score_tx: ScoreTx,
    player: ActorRef<PlayerMsg>,
}

impl HostState {
    pub fn new(dealer: DealerTx, score_tx: ScoreTx, player: ActorRef<PlayerMsg>) -> HostState {
        Self {
            dealer,
            score_tx,
            player,
        }
    }
}

#[derive(RactorMessage, PartialEq, PartialOrd)]
pub enum HostMsg {
    Start,
}

#[ractor::async_trait]
impl Actor for HostFsm {
    actor_types!(HostMsg, HostState, HostState);

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            HostMsg::Start => {
                let cards = ractor::call!(
                    state.dealer,
                    crate::actors::dealer::DealerMsg::DealRedCards,
                    2
                )?;
                println!("{:?}", cards);
            }
        }
        Ok(())
    }
}

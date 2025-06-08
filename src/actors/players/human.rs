use apples_core::{
    cards::{green_card::GreenCard, red_card::RedCard},
    player::player::{PlayerHand, PlayerId},
    protocol::message::GameMessage,
};
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use ractor_cluster::RactorMessage;

#[derive(RactorMessage)]
pub enum HumanPlayerMsg {
    DealCard(RedCard),
    RequestCardChoice(GreenCard, RpcReplyPort<Option<usize>>),
    RequestJudgeChoice(Vec<RedCard>, GreenCard, RpcReplyPort<Option<usize>>),
}

pub struct HumanPlayer;

pub struct HumanPlayerState {
    hand: Vec<RedCard>,
}

#[ractor::async_trait]
impl Actor for HumanPlayer {
    actor_types!(HumanPlayerMsg, HumanPlayerState, ());
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(HumanPlayerState { hand: Vec::new() })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            HumanPlayerMsg::DealCard(card) => {
                state.hand.push(card);
            }

            HumanPlayerMsg::RequestCardChoice(green_card, reply) => {}

            HumanPlayerMsg::RequestJudgeChoice(options, green_card, reply) => {}
        }
        Ok(())
    }
}

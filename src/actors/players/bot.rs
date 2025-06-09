use crate::players::bot_player::Bot;
use apples_core::{cards::green_card::GreenCard, cards::red_card::RedCard};
use apples_utils::actor_types;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use ractor_cluster::RactorMessage;
#[derive(RactorMessage)]
pub enum BotMsg {
    DealCard(RedCard),
    RequestCardChoice(GreenCard, RpcReplyPort<Option<usize>>),
    RequstJudgeChoice(Vec<RedCard>, GreenCard, RpcReplyPort<Option<usize>>),
}

pub struct BotActor;

pub struct BotState {
    bot: Bot,
}
#[ractor::async_trait]
impl Actor for BotActor {
    actor_types!(BotMsg, BotState, apples_core::player::player::PlayerId);

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<BotState, ActorProcessingErr> {
        Ok(BotState {
            bot: Bot::new(args),
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            BotMsg::DealCard(card) => {
                if let Err(e) = state.bot.add_card(card) {
                    eprintln!("Couldn't deal card to bot {}", e);
                }
            }
            BotMsg::RequestCardChoice(_green, reply) => {
                //TODO: alter this with choice provider somehow
                reply.send(Some(0))?;
            }
            BotMsg::RequstJudgeChoice(_choices, _green_card, reply) => {
                reply.send(Some(0))?;
            }
        }
        Ok(())
    }
}

use {
    crate::deck_handler::DeckHandler,
    apples_core::cards::{green_card::GreenCard, red_card::RedCard},
    apples_utils::actor_types,
    ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort},
    ractor_cluster::RactorMessage,
};

#[derive(RactorMessage)]
pub enum DealerMsg {
    DealRedCards(usize, RpcReplyPort<Vec<RedCard>>),
    DealGreenCards(usize, RpcReplyPort<Vec<GreenCard>>),
}

/// Alias for dealer
pub type DealerTx = ActorRef<DealerMsg>;

pub struct DealerState {
    deck_handler: DeckHandler,
}

pub struct DealerActor;

#[ractor::async_trait]
impl Actor for DealerActor {
    actor_types!(DealerMsg, DealerState, DeckHandler);

    async fn pre_start(
        &self,
        _myself: ActorRef<DealerMsg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(DealerState { deck_handler: args })
    }

    async fn handle(
        &self,
        _myself: ActorRef<DealerMsg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            Self::Msg::DealRedCards(number_cards, reply) => {
                let mut vec = Vec::new();
                for _ in 0..number_cards {
                    if let Some(card) = state.deck_handler.get_red_card() {
                        vec.push(card);
                    }
                }
                reply.send(vec)?;
            }
            Self::Msg::DealGreenCards(number_cards, reply) => {
                let mut vec = Vec::new();

                for _ in 0..number_cards {
                    if let Some(card) = state.deck_handler.get_green_card() {
                        vec.push(card);
                    }
                }
                reply.send(vec)?;
            }
        }
        Ok(())
    }
}

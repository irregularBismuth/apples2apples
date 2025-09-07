use crate::deck_handler::DeckHandler;
use ::ractor::RpcReplyPort;
use actor_macros::{actor, actor_handle, actor_pre_start};
use apples_core::{cards::card::Card, GreenCard, GreenDeck, RedCard, RedDeck};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DealError {
    #[error("not enough cards: requested {requested}, available {available}")]
    OutOfCards { requested: usize, available: usize },
    #[error("deck is empty")]
    EmptyDeck,
}

pub(crate) type DealResult<T> = std::result::Result<T, DealError>;

pub(crate) enum DealerMsg {
    RequestGreenCard(usize, RpcReplyPort<DealResult<Vec<GreenCard>>>),
    RequestRedCard(usize, RpcReplyPort<DealResult<Vec<RedCard>>>),
    Shuffle,
}

#[actor(
    msg=DealerMsg,
    state=DealerState,
    args=DeckHandler,
    pre_start = on_start
)]
pub(crate) struct DealerActor;

pub(crate) struct DealerState {
    deck_handler: DeckHandler,
}

impl DealerActor {
    actor_pre_start!({ Ok(DealerState { deck_handler: args }) });
    actor_handle!({
        match msg {
            DealerMsg::RequestGreenCard(amount, reply) => {
                let cards: Vec<Option<GreenCard>> = (0..amount)
                    .map(|_| state.deck_handler.get_green_card())
                    .collect();
            }
            DealerMsg::RequestRedCard(amount, reply) => {
                let cards: Vec<Option<RedCard>> = (0..amount)
                    .map(|_| state.deck_handler.get_red_card())
                    .collect();
            }
            DealerMsg::Shuffle => state.deck_handler.shuffle(),
        }
        Ok(())
    });
}

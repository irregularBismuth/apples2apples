use crate::deck_handler::DeckHandler;
use ::ractor::{ActorProcessingErr, RpcReplyPort};
use apples_core::{cards::card::Card, GreenCard, GreenDeck, RedCard, RedDeck};
use core::num::NonZeroUsize;
use dsl_ractor::{actor, actor_handle, actor_pre_start};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DealError {
    #[error("not enough cards: requested {requested}, available {available}")]
    OutOfCards { requested: usize, available: usize },
    #[error("deck is empty")]
    EmptyDeck,
    #[error("invalid request: cannot deal {amount} cards")]
    InvalidAmount { amount: usize },
}

pub type DealResult<T> = std::result::Result<T, DealError>;

#[derive(Debug)]
pub enum DealerMsg {
    DealGreenCards(NonZeroUsize, RpcReplyPort<DealResult<Vec<GreenCard>>>),
    DealRedCards(NonZeroUsize, RpcReplyPort<DealResult<Vec<RedCard>>>),
    Shuffle,
    GetDeckSizes(RpcReplyPort<(usize, usize)>), // (red_size, green_size)
}

#[actor(
    msg = DealerMsg,
    state = DealerState,
    args = DeckHandler,
)]
pub(crate) struct Dealer;

pub(crate) struct DealerState {
    deck_handler: DeckHandler,
}

impl Dealer {
    actor_pre_start!(Ok(DealerState { deck_handler: args }));

    actor_handle!({
        match msg {
            DealerMsg::DealGreenCards(amount, reply) => {
                let result =
                    Self::deal_cards(amount, &mut state.deck_handler, |h| h.get_green_card());
                if let Err(e) = reply.send(result) {
                    return Err(ActorProcessingErr::from(format!(
                        "Failed to send reply: {e}"
                    )));
                }
            }
            DealerMsg::DealRedCards(amount, reply) => {
                let result =
                    Self::deal_cards(amount, &mut state.deck_handler, |h| h.get_red_card());
                if let Err(e) = reply.send(result) {
                    return Err(ActorProcessingErr::from(format!(
                        "Failed to send reply: {e}"
                    )));
                }
            }
            DealerMsg::Shuffle => {
                state.deck_handler.shuffle();
                tracing::info!("Shuffled decks");
            }
            DealerMsg::GetDeckSizes(reply) => {
                let sizes = (
                    state.deck_handler.red_card_deck_size(),
                    state.deck_handler.green_card_deck_size(),
                );

                if let Err(e) = reply.send(sizes) {
                    return Err(ActorProcessingErr::from(format!(
                        "Failed to send deck sizes: {e}"
                    )));
                }
            }
        }
        Ok(())
    });
}

impl Dealer {
    fn deal_cards<T, F>(
        amount: NonZeroUsize,
        handler: &mut DeckHandler,
        mut draw_fn: F,
    ) -> DealResult<Vec<T>>
    where
        T: Card,
        F: FnMut(&mut DeckHandler) -> Option<T>,
    {
        let expected = amount.get();
        let cards: Vec<T> = (0..expected).map_while(|_| draw_fn(handler)).collect();

        if cards.len() < expected {
            Err(DealError::OutOfCards {
                requested: expected,
                available: cards.len(),
            })
        } else {
            Ok(cards)
        }
    }
}

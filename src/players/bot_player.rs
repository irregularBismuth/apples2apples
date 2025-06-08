use apples_core::player::{
    base_player::BasePlayer,
    player::{ChoiceProvider, PlayerHand, PlayerId},
};

pub struct Bot {
    player: BasePlayer,
}

impl Bot {
    pub fn new(id: usize) -> Bot {
        Self {
            player: BasePlayer::new(PlayerId(id)),
        }
    }
}

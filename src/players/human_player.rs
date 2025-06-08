use apples_core::player::{
    base_player::BasePlayer,
    player::{ChoiceProvider, PlayerId},
};

pub struct Human {
    player: BasePlayer,
}

impl Human {
    pub fn new(id: usize) -> Human {
        Self {
            player: BasePlayer::new(PlayerId(id)),
        }
    }
}

use apples_core::{
    player::{
        base_player::BasePlayer,
        player::{ChoiceProvider, PlayerId},
    },
    GreenCard, RedCard,
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

#[ractor::async_trait]
impl ChoiceProvider for Human {
    async fn choose_card(&self, green_card: &GreenCard) -> Option<usize> {
        Some(0)
    }

    async fn judge_cards(&self, _options: &[RedCard], _green_card: &GreenCard) -> Option<usize> {
        Some(0)
    }
}

use apples_core::{
    cards::green_card::GreenCard,
    cards::red_card::RedCard,
    player::{
        base_player::BasePlayer,
        player::{ChoiceProvider, PlayerId},
    },
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
    /// Bots just choose the first card available
    async fn choose_card(&self, _hand: &[RedCard], _green_card: &GreenCard) -> Option<usize> {
        Some(0)
    }

    /// Bots just choose the first card available
    async fn judge_cards(&self, _options: &[RedCard], _green_card: &GreenCard) -> Option<usize> {
        Some(0)
    }
}

use apples_core::{
    cards::green_card::GreenCard,
    cards::red_card::RedCard,
    player::{
        base_player::BasePlayer,
        player::{ChoiceProvider, PlayerHand, PlayerId},
    },
};

pub struct Bot {
    player: BasePlayer,
}

impl Bot {
    pub fn new(id: PlayerId) -> Bot {
        Self {
            player: BasePlayer::new(id),
        }
    }

    pub fn add_card(&mut self, red: RedCard) -> anyhow::Result<()> {
        self.player.hand.add_card(red);
        Ok(())
    }
}

#[ractor::async_trait]
impl ChoiceProvider for Bot {
    /// Bots just choose the first card available
    async fn choose_card(&self, _hand: &[RedCard], _green_card: &GreenCard) -> Option<usize> {
        Some(0)
    }

    /// Bots just choose the first card available
    async fn judge_cards(&self, _options: &[RedCard], _green_card: &GreenCard) -> Option<usize> {
        Some(0)
    }
}

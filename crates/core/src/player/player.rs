use {
    crate::cards::{green_card::GreenCard, red_card::RedCard},
    async_trait::async_trait,
    std::fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub struct PlayerId(pub usize);

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerID: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerHand {
    cards: Vec<RedCard>,
}

impl PlayerHand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: RedCard) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self, index: usize) -> Option<RedCard> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn get_cards(&self) -> &[RedCard] {
        &self.cards
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[async_trait]
pub trait ChoiceProvider: Send + Sync {
    async fn choose_card(&self, green_card: &GreenCard) -> Option<usize>;
    async fn judge_cards(&self, options: &[RedCard], green_card: &GreenCard) -> Option<usize>;
}

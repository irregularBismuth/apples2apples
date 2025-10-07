use {crate::cards::red_card::RedCard, std::fmt};

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
    #[inline]
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    #[inline]
    pub fn add_card(&mut self, card: RedCard) {
        self.cards.push(card);
    }

    #[inline]
    pub fn remove_card(&mut self, index: usize) -> Option<RedCard> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_cards(&self) -> &[RedCard] {
        &self.cards
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

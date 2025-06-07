use {super::player::PlayerId, crate::deck::red_deck::RedDeck};

pub struct BasePlayer {
    pub id: PlayerId,
    pub hand: RedDeck,
    pub score: usize,
}

impl BasePlayer {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            hand: RedDeck::new(),
            score: 0,
        }
    }
}

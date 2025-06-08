use super::player::{PlayerHand, PlayerId};

pub struct BasePlayer {
    pub id: PlayerId,
    pub hand: PlayerHand,
    pub score: usize,
}

impl BasePlayer {
    pub fn new(id: PlayerId) -> Self {
        Self {
            id,
            hand: PlayerHand::new(),
            score: 0,
        }
    }
}

use {
    crate::cards::card::Card,
    itertools::Itertools,
    rand::seq::SliceRandom,
    serde::{de::DeserializeOwned, Deserialize, Serialize},
};

/// Struct for Deck that takes the Card Trait and holds a vector of cards
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Deck<T: Card> {
    cards: Vec<T>,
}

impl<T: Card> Default for Deck<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Card> Deck<T> {
    /// Creates an empty deck
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    /// shuffle the deck
    #[inline]
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Tries to remove the card at `index`, returning `None` if the index is out of bounds.
    #[inline]
    pub fn draw_index(&mut self, index: usize) -> Option<T> {
        (index < self.cards.len()).then(|| self.cards.remove(index))
    }

    /// returns an option to an card
    #[inline]
    pub fn draw_card(&mut self) -> Option<T> {
        self.cards.pop()
    }
    /// adds a card to the deck
    #[inline]
    pub fn add_card(&mut self, card: T) {
        self.cards.push(card)
    }
    /// returns the amount of cards left inside deck
    #[inline]
    pub fn deck_size(&self) -> usize {
        self.cards.len()
    }
    /// Checks if the input deck is a permutation of the other deck
    pub fn is_permutation(&self, other: &Deck<T>) -> bool {
        let perm = self.cards.iter().counts() == other.cards.iter().counts();
        let different = self.cards != other.cards;
        perm && different
    }
    /// returns a copy of the underlying deck   
    #[inline]
    pub fn get_cards(&self) -> Vec<T> {
        self.cards.clone()
    }

    #[inline]
    pub fn extend(&mut self, deck: Deck<T>) {
        self.cards.extend(deck.cards)
    }
}

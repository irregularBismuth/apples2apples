use {
    super::super::cards::card::Card,
    itertools::Itertools,
    rand::seq::SliceRandom,
    serde::{Deserialize, Serialize},
    std::hash::Hash,
};

/// Struct for Deck that takes the Card Trait and holds a vector of cards
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Deck<T: Card> {
    cards: Vec<T>,
}

impl<T: Card + Clone + Eq + Hash> Deck<T> {
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
    /// Tries to remove the `card` given the index panics if the index
    #[inline]
    pub fn draw_index(&mut self, index: usize) -> T {
        self.cards.remove(index)
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

use apples_core::{
    cards::{green_card::GreenCard, red_card::RedCard},
    deck::{green_deck::GreenDeck, red_deck::RedDeck},
};
use apples_utils::deck_loader::load_deck;

/// DeckHandler class that holds a green & a red deck
pub struct DeckHandler {
    red_deck: RedDeck,
    green_deck: GreenDeck,
}

impl DeckHandler {
    /// Create a empty green and red deck and return the instance of the deck handler
    pub fn new() -> Self {
        Self {
            red_deck: RedDeck::new(),
            green_deck: GreenDeck::new(),
        }
    }

    /// Insert a red card to the deck
    pub fn insert_red_card(&mut self, card: RedCard) {
        self.red_deck.add_card(card);
    }

    /// Insert a green card to the deck
    pub fn insert_green_card(&mut self, card: GreenCard) {
        self.green_deck.add_card(card);
    }

    /// Retrieve card from the green deck
    pub fn get_green_card(&mut self) -> Option<GreenCard> {
        self.green_deck.draw_card()
    }
    /// Retrieve card from the red deck
    pub fn get_red_card(&mut self) -> Option<RedCard> {
        self.red_deck.draw_card()
    }

    /// Shuffle the decks
    pub fn shuffle(&mut self) {
        self.red_deck.shuffle();
        self.green_deck.shuffle();
    }
    /// Load the decks
    pub async fn load_decks(
        &mut self,
        red_file_path: std::path::PathBuf,
        green_file_path: std::path::PathBuf,
    ) {
        let red_deck: RedDeck = load_deck::<RedCard, _>(red_file_path)
            .await
            .expect("failed to load red deck");
        let green_deck: GreenDeck = load_deck::<GreenCard, _>(green_file_path)
            .await
            .expect("failed to load green deck");

        self.red_deck.extend(red_deck);
        self.green_deck.extend(green_deck);
    }

    /// Return the deck size of the green deck
    pub fn green_card_deck_size(&self) -> usize {
        self.green_deck.deck_size()
    }
    /// Return the deck size of the red deck
    pub fn red_card_deck_size(&self) -> usize {
        self.red_deck.deck_size()
    }
}

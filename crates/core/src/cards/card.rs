/// Defines the trait Card which requires the 3 getter functions to be implemented
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct BaseCard {
    name_: String,
    text_: String,
    id_: usize,
}

/// The base trait for a card needs to implment the three functions
/// get_card_id
/// get_card_name
/// get_card_text
pub trait Card: Clone + Eq + std::hash::Hash {
    fn get_card_id(&self) -> usize;
    fn get_card_name(&self) -> &str;
    fn get_card_text(&self) -> &str;
}

impl Card for BaseCard {
    fn get_card_id(&self) -> usize {
        self.id_
    }
    fn get_card_name(&self) -> &str {
        &self.name_
    }
    fn get_card_text(&self) -> &str {
        &self.text_
    }
}

impl BaseCard {
    /// Creates the basecard with corresponding input
    pub fn new(name_: String, text_: String, id_: usize) -> Self {
        Self { name_, text_, id_ }
    }
}

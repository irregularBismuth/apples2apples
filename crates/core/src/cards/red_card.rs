use {
    super::card::BaseCard,
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct RedCard(BaseCard);

impl Card for RedCard {
    fn get_card_id(&self) -> usize {
        self.0.get_card_id()
    }

    fn get_card_name(&self) -> &str {
        self.0.get_card_name()
    }

    fn get_card_text(&self) -> &str {
        self.0.get_card_text()
    }
}

impl RedCard {
    /// Creates a new red card with corresponding input parameters
    /// name_ : String,
    /// text_ : String,
    /// id_ : usize
    pub fn new(name_: String, text_: String, id_: usize) -> Self {
        Self(BaseCard::new(name_, text_, id_))
    }
}

/// Implement the display trait for easily displaying the RedCard
impl fmt::Display for RedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Redcard: ( \n name {} \n text {} \n id {} )",
            self.get_card_name(),
            self.get_card_text(),
            self.get_card_id()
        )
    }
}

impl From<BaseCard> for RedCard {
    fn from(base: BaseCard) -> Self {
        RedCard::new(
            base.get_card_name().to_string(),
            base.get_card_text().to_string(),
            base.get_card_id(),
        )
    }
}

pub use super::card::Card;

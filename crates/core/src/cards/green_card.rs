use {
    super::card::BaseCard,
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GreenCard(BaseCard);

impl Card for GreenCard {
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

impl GreenCard {
    /// Creates a new Green card with corresponding input parameters
    pub fn new(name_: String, text_: String, id_: usize) -> Self {
        Self(BaseCard::new(name_, text_, id_))
    }
}

/// Implement the display trait for easily displaying the RedCard
impl fmt::Display for GreenCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Greencard: ( \n name {} \n text {} \n id {} )",
            self.get_card_name(),
            self.get_card_text(),
            self.get_card_id()
        )
    }
}

impl From<BaseCard> for GreenCard {
    fn from(base: BaseCard) -> Self {
        GreenCard::new(
            base.get_card_name().to_string(),
            base.get_card_text().to_string(),
            base.get_card_id(),
        )
    }
}
pub use super::card::Card;

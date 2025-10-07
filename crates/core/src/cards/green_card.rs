use {
    super::card::{BaseCard, Card, CardId},
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct GreenCard(BaseCard);

impl GreenCard {
    pub fn new(
        id: impl Into<CardId>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self(BaseCard::new(id, name, description))
    }

    #[inline]
    pub fn base(&self) -> &BaseCard {
        &self.0
    }
}

impl Card for GreenCard {
    #[inline]
    fn id(&self) -> CardId {
        self.0.id()
    }

    #[inline]
    fn name(&self) -> &str {
        self.0.name()
    }

    #[inline]
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl fmt::Display for GreenCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Green card: (\n name {}\n text {}\n id {} )",
            self.name(),
            self.description(),
            self.id().value()
        )
    }
}

impl From<BaseCard> for GreenCard {
    #[inline]
    fn from(base: BaseCard) -> Self {
        Self(base)
    }
}

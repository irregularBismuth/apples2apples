use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Debug, hash::Hash};

/// Identifier for any card variant.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardId(usize);

impl CardId {
    /// Create a new `CardId`.
    #[inline]
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    /// Retrieve the inner value.
    #[inline]
    pub fn value(self) -> usize {
        self.0
    }
}

impl From<u32> for CardId {
    #[inline]
    fn from(id: u32) -> Self {
        CardId::new(id as usize)
    }
}

impl From<usize> for CardId {
    #[inline]
    fn from(id: usize) -> Self {
        CardId::new(id)
    }
}

impl From<CardId> for usize {
    #[inline]
    fn from(id: CardId) -> Self {
        id.value()
    }
}

/// Base data shared across concrete card variants.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct BaseCard {
    id: CardId,
    name: String,
    description: String,
}

impl BaseCard {
    pub fn new(
        id: impl Into<CardId>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        }
    }

    #[inline]
    pub fn id(&self) -> CardId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for BaseCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} — {} (id: {})",
            self.name(),
            self.description(),
            self.id().value()
        )
    }
}

/// Common behaviour shared by all card variants.
pub trait Card: Debug + Clone + Eq + Hash {
    fn id(&self) -> CardId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

impl Card for BaseCard {
    #[inline]
    fn id(&self) -> CardId {
        self.id()
    }

    #[inline]
    fn name(&self) -> &str {
        self.name()
    }

    #[inline]
    fn description(&self) -> &str {
        self.description()
    }
}

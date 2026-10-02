use crate::{Consequence, EventId};

/// Narrative event and its ordered consequences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeEvent {
    /// Event ID.
    pub id: EventId,
    /// Name.
    pub name: String,
    /// Consequences.
    pub consequences: Vec<Consequence>,
}

impl NarrativeEvent {
    /// Creates an event.
    pub fn new(id: EventId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            consequences: Vec::new(),
        }
    }
}

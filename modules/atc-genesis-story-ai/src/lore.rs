use crate::{LoreId, StoryError, StoryResult};
use std::collections::BTreeMap;

/// Canonical lore entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoreEntry {
    /// Lore ID.
    pub id: LoreId,
    /// Title.
    pub title: String,
    /// Canonical text.
    pub text: String,
}
/// Canonical lore database.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoreDatabase {
    /// Entries.
    pub entries: BTreeMap<LoreId, LoreEntry>,
}
impl LoreDatabase {
    /// Validates lore.
    pub fn validate(&self) -> StoryResult<()> {
        for e in self.entries.values() {
            if e.title.trim().is_empty() || e.text.trim().is_empty() { return Err(StoryError::InvalidState(format!("lore {} incomplete", e.id.value()))); }
        }
        Ok(())
    }
}

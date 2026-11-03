use crate::{MemoryId, StoryError, StoryResult};
use std::collections::BTreeMap;

/// Narrative memory type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    /// Player decision.
    PlayerDecision,
    /// World event.
    Event,
    /// Discovery.
    Discovery,
    /// Relationship change.
    Relationship,
    /// Lore discovery.
    Lore,
}

/// Persistent narrative memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    /// Memory ID.
    pub id: MemoryId,
    /// Kind.
    pub kind: MemoryKind,
    /// Subject.
    pub subject: String,
    /// Value.
    pub value: String,
}

/// Memory store.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryStore {
    /// Memories.
    pub memories: BTreeMap<MemoryId, Memory>,
}

impl MemoryStore {
    /// Inserts a memory.
    pub fn insert(&mut self, memory: Memory) -> StoryResult<()> {
        if self.memories.contains_key(&memory.id) {
            return Err(StoryError::AlreadyExists(format!(
                "memory {}",
                memory.id.value()
            )));
        }
        if memory.subject.trim().is_empty() {
            return Err(StoryError::InvalidState(
                "memory subject is empty".into(),
            ));
        }
        self.memories.insert(memory.id, memory);
        Ok(())
    }
}

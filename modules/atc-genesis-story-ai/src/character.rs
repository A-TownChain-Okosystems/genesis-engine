use crate::{CharacterId, StoryError, StoryResult};
use std::collections::BTreeMap;

/// Relationship score in the inclusive range -100..=100.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relationship(pub i32);
impl Relationship {
    /// Creates a clamped score.
    pub fn new(value: i32) -> Self { Self(value.clamp(-100, 100)) }
    /// Changes the score.
    pub fn change(&mut self, delta: i32) { self.0 = (self.0 + delta).clamp(-100, 100); }
}

/// Runtime character state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterState {
    /// Identifier.
    pub id: CharacterId,
    /// Display name.
    pub name: String,
    /// Life state.
    pub alive: bool,
    /// Relationships.
    pub relationships: BTreeMap<CharacterId, Relationship>,
    /// Reputation.
    pub reputation: i32,
    /// Learned lore IDs.
    pub known_lore: Vec<u64>,
}
impl CharacterState {
    /// Creates a character.
    pub fn new(id: CharacterId, name: impl Into<String>) -> Self {
        Self { id, name: name.into(), alive: true, relationships: BTreeMap::new(), reputation: 0, known_lore: Vec::new() }
    }
    /// Changes a relationship.
    pub fn change_relationship(&mut self, other: CharacterId, delta: i32) { self.relationships.entry(other).or_insert(Relationship::new(0)).change(delta); }
    /// Reads a relationship.
    pub fn relationship(&self, other: CharacterId) -> i32 { self.relationships.get(&other).map_or(0, |r| r.0) }
    /// Learns lore.
    pub fn learn_lore(&mut self, lore: u64) { if !self.known_lore.contains(&lore) { self.known_lore.push(lore); self.known_lore.sort_unstable(); } }
    /// Validates invariants.
    pub fn validate(&self) -> StoryResult<()> {
        if self.name.trim().is_empty() { return Err(StoryError::InvalidState("character name is empty".into())); }
        if !(-100..=100).contains(&self.reputation) { return Err(StoryError::InvalidState("reputation out of range".into())); }
        Ok(())
    }
}

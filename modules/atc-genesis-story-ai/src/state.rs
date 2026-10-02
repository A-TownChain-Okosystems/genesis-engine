use crate::{CharacterId, CharacterState, QuestId, QuestState, StoryError, StoryResult};
use std::collections::{BTreeMap, BTreeSet};

/// Mutable deterministic narrative state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldState {
    /// Characters.
    pub characters: BTreeMap<CharacterId, CharacterState>,
    /// Quest state.
    pub quests: BTreeMap<QuestId, QuestState>,
    /// Boolean flags.
    pub flags: BTreeMap<String, bool>,
    /// Integer counters.
    pub counters: BTreeMap<String, i64>,
    /// Completed quest IDs.
    pub completed_quests: BTreeSet<u64>,
    /// Failed quest IDs.
    pub failed_quests: BTreeSet<u64>,
    /// Current scene.
    pub current_scene: Option<u64>,
    /// Simulation tick.
    pub tick: u64,
}
impl Default for WorldState {
    fn default() -> Self { Self { characters: BTreeMap::new(), quests: BTreeMap::new(), flags: BTreeMap::new(), counters: BTreeMap::new(), completed_quests: BTreeSet::new(), failed_quests: BTreeSet::new(), current_scene: None, tick: 0 } }
}
impl WorldState {
    /// Validates state invariants.
    pub fn validate(&self) -> StoryResult<()> {
        for c in self.characters.values() { c.validate()?; }
        if self.completed_quests.intersection(&self.failed_quests).next().is_some() { return Err(StoryError::InvalidState("quest is both completed and failed".into())); }
        Ok(())
    }
}

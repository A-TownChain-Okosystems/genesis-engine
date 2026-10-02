use crate::{CharacterId, QuestId, StoryError, StoryResult, WorldState};

/// Deterministic world-state mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consequence {
    /// Change relationship.
    Relationship { character: CharacterId, other: CharacterId, delta: i32 },
    /// Change reputation.
    Reputation { character: CharacterId, delta: i32 },
    /// Set a boolean flag.
    SetFlag { key: String, value: bool },
    /// Add a signed counter.
    AddCounter { key: String, delta: i64 },
    /// Complete a quest.
    CompleteQuest(QuestId),
    /// Fail a quest.
    FailQuest(QuestId),
}
impl Consequence {
    /// Applies one consequence.
    pub fn apply(&self, state: &mut WorldState) -> StoryResult<()> {
        match self {
            Self::Relationship { character, other, delta } => {
                let c = state.characters.get_mut(character).ok_or_else(|| StoryError::NotFound(format!("character {}", character.value())))?;
                c.change_relationship(*other, *delta);
            }
            Self::Reputation { character, delta } => {
                let c = state.characters.get_mut(character).ok_or_else(|| StoryError::NotFound(format!("character {}", character.value())))?;
                c.reputation = (c.reputation + delta).clamp(-100, 100);
            }
            Self::SetFlag { key, value } => { state.flags.insert(key.clone(), *value); }
            Self::AddCounter { key, delta } => {
                let old = state.counters.get(key).copied().unwrap_or(0);
                state.counters.insert(key.clone(), old.checked_add(*delta).ok_or_else(|| StoryError::InvalidState("counter overflow".into()))?);
            }
            Self::CompleteQuest(id) => {
                state.completed_quests.insert(id.value()); state.failed_quests.remove(&id.value());
                if let Some(q) = state.quests.get_mut(id) { q.status = crate::QuestStatus::Completed; }
            }
            Self::FailQuest(id) => {
                state.failed_quests.insert(id.value()); state.completed_quests.remove(&id.value());
                if let Some(q) = state.quests.get_mut(id) { q.status = crate::QuestStatus::Failed; }
            }
        }
        Ok(())
    }
}

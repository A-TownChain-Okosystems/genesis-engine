use crate::{QuestId, QuestStepId, StoryError, StoryResult};
use std::collections::BTreeSet;

/// Quest lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestStatus {
    /// Available.
    Available,
    /// Active.
    Active,
    /// Completed.
    Completed,
    /// Failed.
    Failed,
    /// Abandoned.
    Abandoned,
    /// Expired.
    Expired,
}

/// Quest objective.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestStep {
    /// Step ID.
    pub id: QuestStepId,
    /// Description.
    pub description: String,
}

/// Quest definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quest {
    /// Quest ID.
    pub id: QuestId,
    /// Name.
    pub name: String,
    /// Ordered objectives.
    pub steps: Vec<QuestStep>,
}

/// Runtime quest state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestState {
    /// Status.
    pub status: QuestStatus,
    /// Completed objectives.
    pub completed_steps: BTreeSet<QuestStepId>,
}

impl Default for QuestState {
    fn default() -> Self {
        Self {
            status: QuestStatus::Available,
            completed_steps: BTreeSet::new(),
        }
    }
}

impl Quest {
    /// Validates a quest definition.
    pub fn validate(&self) -> StoryResult<()> {
        if self.name.trim().is_empty() {
            return Err(StoryError::InvalidState("quest name is empty".into()));
        }
        if self.steps.is_empty() {
            return Err(StoryError::InvalidState(format!(
                "quest {} has no steps",
                self.id.value()
            )));
        }
        if self
            .steps
            .iter()
            .any(|s| s.description.trim().is_empty())
        {
            return Err(StoryError::InvalidState("empty quest step".into()));
        }
        Ok(())
    }
}

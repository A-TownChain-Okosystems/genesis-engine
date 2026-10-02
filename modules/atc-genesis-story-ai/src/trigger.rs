use crate::{Consequence, StoryError, StoryResult, TriggerId, WorldState};
use std::collections::{BTreeMap, BTreeSet};

/// Trigger condition evaluated against deterministic state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerCondition {
    /// Boolean flag equals a value.
    Flag { key: String, value: bool },
    /// Counter is at least a value.
    CounterAtLeast { key: String, value: i64 },
    /// Scene is active.
    Scene(u64),
    /// All conditions must match.
    All(Vec<TriggerCondition>),
    /// At least one condition must match.
    Any(Vec<TriggerCondition>),
}

impl TriggerCondition {
    /// Evaluates the condition.
    pub fn matches(&self, state: &WorldState) -> bool {
        match self {
            Self::Flag { key, value } => {
                state.flags.get(key).copied() == Some(*value)
            }
            Self::CounterAtLeast { key, value } => {
                state.counters.get(key).copied().unwrap_or(0) >= *value
            }
            Self::Scene(id) => state.current_scene == Some(*id),
            Self::All(xs) => xs.iter().all(|x| x.matches(state)),
            Self::Any(xs) => xs.iter().any(|x| x.matches(state)),
        }
    }
}

/// Runtime trigger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trigger {
    /// Trigger ID.
    pub id: TriggerId,
    /// Condition.
    pub condition: TriggerCondition,
    /// Consequences.
    pub consequences: Vec<Consequence>,
    /// Fire once only.
    pub one_shot: bool,
}

/// Eligible trigger consequences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerBatch {
    /// Trigger IDs that fired.
    pub fired: BTreeSet<TriggerId>,
    /// Ordered consequences.
    pub consequences: Vec<Consequence>,
}

/// Trigger registry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TriggerRegistry {
    pub(crate) triggers: BTreeMap<TriggerId, Trigger>,
    pub(crate) fired: BTreeSet<TriggerId>,
}

impl TriggerRegistry {
    /// Inserts a trigger.
    pub fn insert(&mut self, trigger: Trigger) -> StoryResult<()> {
        if trigger.consequences.is_empty() {
            return Err(StoryError::InvalidState(
                "trigger has no consequences".into(),
            ));
        }
        if self.triggers.contains_key(&trigger.id) {
            return Err(StoryError::AlreadyExists(format!(
                "trigger {}",
                trigger.id.value()
            )));
        }
        self.triggers.insert(trigger.id, trigger);
        Ok(())
    }

    /// Evaluates triggers without committing their fired state.
    pub fn collect(&self, state: &WorldState) -> TriggerBatch {
        let mut fired = BTreeSet::new();
        let mut consequences = Vec::new();

        for (id, trigger) in &self.triggers {
            if trigger.one_shot && self.fired.contains(id) {
                continue;
            }
            if trigger.condition.matches(state) {
                consequences.extend(trigger.consequences.clone());
                if trigger.one_shot {
                    fired.insert(*id);
                }
            }
        }

        TriggerBatch {
            fired,
            consequences,
        }
    }

    /// Commits one-shot trigger IDs after consequences succeed.
    pub fn mark_fired(&mut self, ids: BTreeSet<TriggerId>) {
        self.fired.extend(ids);
    }
}

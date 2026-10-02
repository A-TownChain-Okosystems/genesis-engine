use crate::{
    CharacterState, ChoiceId, Consequence, SceneId, StoryDefinition,
    StoryEngineSnapshot, StoryError, StoryResult, Timeline, TimelineEvent,
    TimelineEventId, TriggerRegistry, WorldState,
};
use std::collections::BTreeSet;

/// Deterministic narrative runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryRuntime {
    /// Static story definition.
    pub definition: StoryDefinition,
    /// Mutable narrative state.
    pub state: WorldState,
    /// Narrative timeline.
    pub timeline: Timeline,
    /// Runtime triggers.
    pub triggers: TriggerRegistry,
    next_timeline_id: u64,
}

impl StoryRuntime {
    /// Creates a runtime after validating static content.
    pub fn new(definition: StoryDefinition) -> StoryResult<Self> {
        definition.validate()?;
        Ok(Self {
            definition,
            state: WorldState::default(),
            timeline: Timeline::default(),
            triggers: TriggerRegistry::default(),
            next_timeline_id: 1,
        })
    }

    /// Adds a character.
    pub fn add_character(&mut self, c: CharacterState) -> StoryResult<()> {
        if self.state.characters.contains_key(&c.id) {
            return Err(StoryError::AlreadyExists(format!(
                "character {}",
                c.id.value()
            )));
        }
        c.validate()?;
        self.state.characters.insert(c.id, c);
        Ok(())
    }

    /// Starts a quest.
    pub fn start_quest(&mut self, id: crate::QuestId) -> StoryResult<()> {
        if !self.definition.quests.contains_key(&id) {
            return Err(StoryError::NotFound(format!(
                "quest {}",
                id.value()
            )));
        }
        let q = self.state.quests.entry(id).or_default();
        if q.status != crate::QuestStatus::Available {
            return Err(StoryError::InvalidTransition(format!(
                "quest {} not available",
                id.value()
            )));
        }
        q.status = crate::QuestStatus::Active;
        Ok(())
    }

    /// Applies consequences atomically.
    pub fn apply_transaction(
        &mut self,
        consequences: &[Consequence],
    ) -> StoryResult<()> {
        let snapshot = self.state.clone();
        for c in consequences {
            if let Err(e) = c.apply(&mut self.state) {
                self.state = snapshot;
                return Err(e);
            }
        }
        if let Err(e) = self.state.validate() {
            self.state = snapshot;
            return Err(e);
        }
        Ok(())
    }

    /// Enters a scene, applying its event and eligible triggers atomically.
    pub fn enter_scene(&mut self, scene: SceneId) -> StoryResult<()> {
        if !self.definition.scenes.contains_key(&scene) {
            return Err(StoryError::NotFound(format!(
                "scene {}",
                scene.value()
            )));
        }

        let state_snapshot = self.state.clone();
        let timeline_snapshot = self.timeline.clone();
        let triggers_snapshot = self.triggers.clone();
        let next_snapshot = self.next_timeline_id;

        let result = (|| {
            self.state.current_scene = Some(scene.value());
            self.state.tick = self
                .state
                .tick
                .checked_add(1)
                .ok_or_else(|| StoryError::InvalidState("tick overflow".into()))?;

            if let Some(event_id) = self.definition.scenes[&scene].event {
                let event = self.definition.events.get(&event_id).ok_or_else(|| {
                    StoryError::NotFound(format!("event {}", event_id.value()))
                })?;
                self.apply_transaction(&event.consequences)?;
            }

            let timeline_id = TimelineEventId::new(self.next_timeline_id);
            self.next_timeline_id = self
                .next_timeline_id
                .checked_add(1)
                .ok_or_else(|| StoryError::InvalidState("timeline ID overflow".into()))?;

            self.timeline.append(TimelineEvent {
                id: timeline_id,
                tick: self.state.tick,
                label: format!("scene:{}", scene.value()),
            })?;

            let eligible = self.triggers.collect(&self.state);
            let fired: BTreeSet<_> = eligible.fired.iter().copied().collect();
            self.apply_transaction(&eligible.consequences)?;
            self.triggers.mark_fired(fired);
            Ok(())
        })();

        if result.is_err() {
            self.state = state_snapshot;
            self.timeline = timeline_snapshot;
            self.triggers = triggers_snapshot;
            self.next_timeline_id = next_snapshot;
        }

        result
    }

    /// Chooses a narrative edge from the current scene.
    pub fn choose(&mut self, choice: ChoiceId) -> StoryResult<SceneId> {
        let current = self
            .state
            .current_scene
            .ok_or_else(|| StoryError::InvalidTransition("no active scene".into()))?;

        let target = self
            .definition
            .graph
            .target(SceneId::new(current), choice)
            .ok_or_else(|| {
                StoryError::InvalidTransition(format!(
                    "choice {} unavailable",
                    choice.value()
                ))
            })?;

        self.enter_scene(target)?;
        Ok(target)
    }

    /// Creates a replay snapshot.
    pub fn snapshot(&self) -> StoryEngineSnapshot {
        StoryEngineSnapshot {
            state: self.state.clone(),
            timeline: self.timeline.clone(),
        }
    }
}

/// Snapshot used by replay and external verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryEngineSnapshot {
    /// World state.
    pub state: WorldState,
    /// Timeline.
    pub timeline: Timeline,
}

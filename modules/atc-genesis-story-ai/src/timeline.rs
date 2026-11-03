use crate::{TimelineEventId, StoryError, StoryResult};

/// Monotonic narrative timeline event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEvent {
    /// Event ID.
    pub id: TimelineEventId,
    /// Logical tick.
    pub tick: u64,
    /// Label.
    pub label: String,
}

/// Deterministic timeline.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Timeline {
    events: Vec<TimelineEvent>,
}

impl Timeline {
    /// Appends an event.
    pub fn append(&mut self, event: TimelineEvent) -> StoryResult<()> {
        if let Some(last) = self.events.last() {
            if event.tick < last.tick {
                return Err(StoryError::InvalidState(
                    "timeline moved backwards".into(),
                ));
            }
        }
        if self.events.iter().any(|e| e.id == event.id) {
            return Err(StoryError::AlreadyExists(format!(
                "timeline event {}",
                event.id.value()
            )));
        }
        self.events.push(event);
        Ok(())
    }

    /// Returns events.
    pub fn events(&self) -> &[TimelineEvent] {
        &self.events
    }
}

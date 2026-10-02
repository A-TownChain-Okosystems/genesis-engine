use crate::{StoryDefinition, StoryError, StoryResult};

/// Structural validation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// Number of scenes.
    pub scenes: usize,
    /// Number of chapters.
    pub chapters: usize,
    /// Number of quests.
    pub quests: usize,
    /// Number of events.
    pub events: usize,
    /// Number of lore entries.
    pub lore_entries: usize,
}

/// Validates a story and returns structural counts.
pub fn validate_story(
    story: &StoryDefinition,
) -> StoryResult<ValidationReport> {
    story.validate()?;
    if story.scenes.is_empty() {
        return Err(StoryError::InvalidState("story has no scenes".into()));
    }
    Ok(ValidationReport {
        scenes: story.scenes.len(),
        chapters: story.chapters.len(),
        quests: story.quests.len(),
        events: story.events.len(),
        lore_entries: story.lore.entries.len(),
    })
}

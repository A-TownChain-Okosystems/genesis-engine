use crate::{DialogueGraph, EventId, LoreDatabase, NarrativeEvent, Quest, QuestId, SceneId, StoryError, StoryGraph, StoryId, StoryResult};
use std::collections::{BTreeMap, BTreeSet};

/// Narrative scene.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scene {
    /// Scene ID.
    pub id: SceneId,
    /// Title.
    pub title: String,
    /// Optional entry event.
    pub event: Option<EventId>,
}
/// Story chapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chapter {
    /// Chapter ID.
    pub id: crate::ChapterId,
    /// Title.
    pub title: String,
    /// Scene IDs.
    pub scenes: Vec<SceneId>,
}
/// Static story definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryDefinition {
    /// Story ID.
    pub id: StoryId,
    /// Title.
    pub title: String,
    /// Chapters.
    pub chapters: BTreeMap<crate::ChapterId, Chapter>,
    /// Scenes.
    pub scenes: BTreeMap<SceneId, Scene>,
    /// Story graph.
    pub graph: StoryGraph,
    /// Narrative events.
    pub events: BTreeMap<EventId, NarrativeEvent>,
    /// Quest definitions.
    pub quests: BTreeMap<QuestId, Quest>,
    /// Dialogue graph.
    pub dialogues: DialogueGraph,
    /// Canonical lore.
    pub lore: LoreDatabase,
}
impl StoryDefinition {
    /// Creates an empty definition.
    pub fn new(id: StoryId, title: impl Into<String>) -> Self { Self { id, title: title.into(), chapters: BTreeMap::new(), scenes: BTreeMap::new(), graph: StoryGraph::new(), events: BTreeMap::new(), quests: BTreeMap::new(), dialogues: DialogueGraph::default(), lore: LoreDatabase::default() } }
    /// Validates static content.
    pub fn validate(&self) -> StoryResult<()> {
        if self.title.trim().is_empty() { return Err(StoryError::InvalidState("story title is empty".into())); }
        let scenes: BTreeSet<_> = self.scenes.keys().copied().collect();
        self.graph.validate(&scenes)?;
        self.dialogues.validate()?;
        self.lore.validate()?;
        for q in self.quests.values() { q.validate()?; }
        for c in self.chapters.values() {
            for s in &c.scenes { if !self.scenes.contains_key(s) { return Err(StoryError::NotFound(format!("scene {}", s.value()))); } }
        }
        for scene in self.scenes.values() {
            if let Some(event) = scene.event {
                if !self.events.contains_key(&event) { return Err(StoryError::NotFound(format!("event {}", event.value()))); }
            }
        }
        Ok(())
    }
}

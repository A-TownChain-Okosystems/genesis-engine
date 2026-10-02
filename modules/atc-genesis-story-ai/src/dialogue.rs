use crate::{CharacterId, ChoiceId, DialogueId, StoryError, StoryResult};
use std::collections::BTreeMap;

/// Dialogue choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogueChoice {
    /// Choice ID.
    pub id: ChoiceId,
    /// Player-visible text.
    pub text: String,
    /// Optional next node.
    pub next: Option<DialogueId>,
}

/// Dialogue node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogueNode {
    /// Node ID.
    pub id: DialogueId,
    /// Speaker.
    pub speaker: CharacterId,
    /// Text.
    pub text: String,
    /// Choices.
    pub choices: Vec<DialogueChoice>,
}

/// Dialogue graph.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DialogueGraph {
    /// Nodes.
    pub nodes: BTreeMap<DialogueId, DialogueNode>,
}
impl DialogueGraph {
    /// Validates dialogue references.
    pub fn validate(&self) -> StoryResult<()> {
        for node in self.nodes.values() {
            if node.text.trim().is_empty() { return Err(StoryError::InvalidState(format!("dialogue {} is empty", node.id.value()))); }
            for c in &node.choices {
                if c.text.trim().is_empty() { return Err(StoryError::InvalidState("dialogue choice is empty".into())); }
                if let Some(next) = c.next {
                    if !self.nodes.contains_key(&next) { return Err(StoryError::InvalidState(format!("missing dialogue {}", next.value()))); }
                }
            }
        }
        Ok(())
    }
}

use crate::{ChoiceId, SceneId, StoryError, StoryResult};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Directed narrative graph.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StoryGraph { edges: BTreeMap<SceneId, BTreeMap<ChoiceId, SceneId>> }
impl StoryGraph {
    /// Creates an empty graph.
    pub fn new() -> Self { Self::default() }
    /// Connects a choice to a target scene.
    pub fn connect(&mut self, from: SceneId, choice: ChoiceId, to: SceneId) { self.edges.entry(from).or_default().insert(choice, to); }
    /// Resolves a choice.
    pub fn target(&self, from: SceneId, choice: ChoiceId) -> Option<SceneId> { self.edges.get(&from).and_then(|m| m.get(&choice)).copied() }
    /// Validates all graph references.
    pub fn validate(&self, scenes: &BTreeSet<SceneId>) -> StoryResult<()> {
        for (from, choices) in &self.edges {
            if !scenes.contains(from) { return Err(StoryError::InvalidState(format!("missing graph source {}", from.value()))); }
            for to in choices.values() {
                if !scenes.contains(to) { return Err(StoryError::InvalidState(format!("missing graph target {}", to.value()))); }
            }
        }
        Ok(())
    }
    /// Returns all reachable scenes.
    pub fn reachable(&self, start: SceneId) -> BTreeSet<SceneId> {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([start]);
        while let Some(node) = queue.pop_front() {
            if !seen.insert(node) { continue; }
            if let Some(next) = self.edges.get(&node) { queue.extend(next.values().copied()); }
        }
        seen
    }
}

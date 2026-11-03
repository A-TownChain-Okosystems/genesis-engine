use crate::{Consequence, ProposalId, StoryError, StoryResult, WorldState};
use std::collections::BTreeMap;

/// Untrusted narrative proposal from an AI provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeProposal {
    /// Proposal ID.
    pub id: ProposalId,
    /// Human-readable rationale.
    pub rationale: String,
    /// Proposed consequences.
    pub consequences: Vec<Consequence>,
}
/// Provider-neutral narrative planner.
pub trait NarrativePlanner {
    /// Creates an untrusted proposal from read-only state.
    fn propose(&self, state: &WorldState) -> NarrativeProposal;
}
/// Deterministic proposal validator.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProposalValidator;
impl ProposalValidator {
    /// Validates a proposal without mutating the runtime.
    pub fn validate(&self, proposal: &NarrativeProposal, state: &WorldState) -> StoryResult<()> {
        if proposal.rationale.trim().is_empty() { return Err(StoryError::InvalidProposal("missing rationale".into())); }
        if proposal.consequences.is_empty() { return Err(StoryError::InvalidProposal("proposal has no consequences".into())); }
        let mut probe = state.clone();
        for consequence in &proposal.consequences { consequence.apply(&mut probe).map_err(|e| StoryError::InvalidProposal(e.to_string()))?; }
        probe.validate().map_err(|e| StoryError::InvalidProposal(e.to_string()))
    }
}
/// Fixed planner for offline tooling and tests.
#[derive(Debug, Clone)]
pub struct FixedPlanner {
    /// Fixed proposal.
    pub proposal: NarrativeProposal,
}
impl NarrativePlanner for FixedPlanner { fn propose(&self, _state: &WorldState) -> NarrativeProposal { self.proposal.clone() } }
/// Proposal provenance store.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProposalBook {
    /// Proposals by ID.
    pub proposals: BTreeMap<ProposalId, NarrativeProposal>,
}
impl ProposalBook {
    /// Records a proposal.
    pub fn insert(&mut self, proposal: NarrativeProposal) -> StoryResult<()> {
        if self.proposals.contains_key(&proposal.id) { return Err(StoryError::AlreadyExists(format!("proposal {}", proposal.id.value()))); }
        self.proposals.insert(proposal.id, proposal); Ok(())
    }
}

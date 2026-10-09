use crate::{
    NarrativeProposal, ProposalValidator, StoryError, StoryResult, StoryRuntime,
};

/// Genesis Engine integration boundary.
#[derive(Debug, Default, Clone, Copy)]
pub struct GenesisStoryAdapter {
    validator: ProposalValidator,
}

impl GenesisStoryAdapter {
    /// Creates the adapter.
    pub const fn new() -> Self {
        Self {
            validator: ProposalValidator,
        }
    }

    /// Validates and atomically commits a proposal.
    pub fn commit_proposal(
        &self,
        runtime: &mut StoryRuntime,
        proposal: &NarrativeProposal,
    ) -> StoryResult<()> {
        self.validator.validate(proposal, &runtime.state)?;
        runtime.apply_transaction(&proposal.consequences)
    }
}

/// Validated command for an engine-side dispatcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedStoryCommand {
    /// Proposal ID.
    pub proposal_id: crate::ProposalId,
    /// Consequences.
    pub consequences: Vec<crate::Consequence>,
}

impl TryFrom<&NarrativeProposal> for ValidatedStoryCommand {
    type Error = StoryError;

    fn try_from(p: &NarrativeProposal) -> Result<Self, Self::Error> {
        if p.consequences.is_empty() {
            return Err(StoryError::InvalidProposal("empty proposal".into()));
        }
        Ok(Self {
            proposal_id: p.id,
            consequences: p.consequences.clone(),
        })
    }
}

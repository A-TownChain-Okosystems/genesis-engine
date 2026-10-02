/// Narrative-domain error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoryError {
    /// Entity already exists.
    AlreadyExists(String),
    /// Entity does not exist.
    NotFound(String),
    /// State invariant failed.
    InvalidState(String),
    /// Transition is not legal.
    InvalidTransition(String),
    /// Proposal failed validation.
    InvalidProposal(String),
    /// Replay diverged from the recorded state.
    ReplayMismatch { expected: u64, actual: u64 },
}
impl std::fmt::Display for StoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists(v) => write!(f, "already exists: {v}"),
            Self::NotFound(v) => write!(f, "not found: {v}"),
            Self::InvalidState(v) => write!(f, "invalid state: {v}"),
            Self::InvalidTransition(v) => write!(f, "invalid transition: {v}"),
            Self::InvalidProposal(v) => write!(f, "invalid proposal: {v}"),
            Self::ReplayMismatch { expected, actual } => write!(f, "replay mismatch: expected {expected}, got {actual}"),
        }
    }
}
impl std::error::Error for StoryError {}
/// Result alias.
pub type StoryResult<T> = Result<T, StoryError>;

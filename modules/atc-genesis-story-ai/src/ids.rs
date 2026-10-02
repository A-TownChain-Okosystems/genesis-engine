use std::marker::PhantomData;

/// Strongly typed deterministic identifier.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T> {
    value: u64,
    marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    /// Creates an identifier.
    pub const fn new(value: u64) -> Self {
        Self { value, marker: PhantomData }
    }

    /// Returns the numeric value.
    pub const fn value(self) -> u64 {
        self.value
    }
}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Id").field(&self.value).finish()
    }
}

macro_rules! ids {
    ($($name:ident),+ $(,)?) => {
        $(pub type $name = Id<$name>;)+
    };
}

ids!(
    StoryId, ChapterId, SceneId, CharacterId, FactionId, LocationId,
    QuestId, QuestStepId, EventId, DialogueId, ChoiceId, LoreId,
    MemoryId, TimelineEventId, ProposalId, TriggerId, TransactionId
);

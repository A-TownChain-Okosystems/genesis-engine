use atc_genesis_story_ai::*;

#[test]
fn invalid_ai_proposal_cannot_mutate_runtime() {
    let mut r =
        StoryRuntime::new(StoryDefinition::new(StoryId::new(1), "AI")).unwrap();
    let proposal = NarrativeProposal {
        id: ProposalId::new(1),
        rationale: "invalid target".into(),
        consequences: vec![Consequence::Reputation {
            character: CharacterId::new(99),
            delta: 10,
        }],
    };
    let before = r.state.clone();

    assert!(GenesisStoryAdapter::new()
        .commit_proposal(&mut r, &proposal)
        .is_err());
    assert_eq!(r.state, before);
}

#[test]
fn valid_proposal_commits() {
    let mut r =
        StoryRuntime::new(StoryDefinition::new(StoryId::new(1), "AI")).unwrap();
    r.add_character(CharacterState::new(CharacterId::new(1), "Hero"))
        .unwrap();

    let proposal = NarrativeProposal {
        id: ProposalId::new(2),
        rationale: "reputation change".into(),
        consequences: vec![Consequence::Reputation {
            character: CharacterId::new(1),
            delta: 10,
        }],
    };

    GenesisStoryAdapter::new()
        .commit_proposal(&mut r, &proposal)
        .unwrap();
    assert_eq!(
        r.state.characters[&CharacterId::new(1)].reputation,
        10
    );
}

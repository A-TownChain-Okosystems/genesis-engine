use atc_genesis_story_ai::*;
#[test]
fn transaction_rolls_back_on_failure() {
    let d=StoryDefinition::new(StoryId::new(1),"Rollback");
    let mut r=StoryRuntime::new(d).unwrap();
    r.add_character(CharacterState::new(CharacterId::new(1),"Hero")).unwrap();
    let result=r.apply_transaction(&[
        Consequence::SetFlag{key:"temporary".into(),value:true},
        Consequence::Reputation{character:CharacterId::new(999),delta:5},
    ]);
    assert!(result.is_err()); assert!(!r.state.flags.contains_key("temporary"));
}
#[test]
fn consequence_order_is_stable() {
    let mut r=StoryRuntime::new(StoryDefinition::new(StoryId::new(1),"Order")).unwrap();
    r.apply_transaction(&[
        Consequence::AddCounter{key:"score".into(),delta:10},
        Consequence::AddCounter{key:"score".into(),delta:-3},
    ]).unwrap();
    assert_eq!(r.state.counters["score"],7);
}

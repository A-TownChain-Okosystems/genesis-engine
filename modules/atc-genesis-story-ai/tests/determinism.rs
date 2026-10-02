use atc_genesis_story_ai::*;

fn build() -> StoryRuntime {
    let a = SceneId::new(1);
    let b = SceneId::new(2);
    let mut d = StoryDefinition::new(StoryId::new(1), "Determinism");

    d.scenes.insert(
        a,
        Scene {
            id: a,
            title: "A".into(),
            event: None,
        },
    );
    d.scenes.insert(
        b,
        Scene {
            id: b,
            title: "B".into(),
            event: None,
        },
    );
    d.graph.connect(a, ChoiceId::new(1), b);
    StoryRuntime::new(d).unwrap()
}

#[test]
fn identical_inputs_have_identical_fingerprints() {
    let mut a = build();
    let mut b = build();

    a.enter_scene(SceneId::new(1)).unwrap();
    b.enter_scene(SceneId::new(1)).unwrap();
    a.choose(ChoiceId::new(1)).unwrap();
    b.choose(ChoiceId::new(1)).unwrap();

    assert_eq!(state_fingerprint(&a), state_fingerprint(&b));
}

#[test]
fn replay_reproduces_state() {
    let mut expected = build();
    expected.enter_scene(SceneId::new(1)).unwrap();
    expected.choose(ChoiceId::new(1)).unwrap();
    let expected_fp = state_fingerprint(&expected);

    let mut actual = build();
    let mut log = ReplayLog::default();
    log.push(ReplayCommand::EnterScene(1));
    log.push(ReplayCommand::Choose(ChoiceId::new(1)));
    log.replay(&mut actual, expected_fp).unwrap();
}

use atc_genesis_story_ai::*;

fn definition() -> StoryDefinition {
    let a = SceneId::new(1);
    let b = SceneId::new(2);
    let mut d = StoryDefinition::new(StoryId::new(1), "Genesis");

    d.scenes.insert(
        a,
        Scene {
            id: a,
            title: "Start".into(),
            event: None,
        },
    );
    d.scenes.insert(
        b,
        Scene {
            id: b,
            title: "Choice".into(),
            event: None,
        },
    );
    d.graph.connect(a, ChoiceId::new(7), b);
    d
}

#[test]
fn choice_advances_story() {
    let mut r = StoryRuntime::new(definition()).unwrap();
    r.enter_scene(SceneId::new(1)).unwrap();
    assert_eq!(
        r.choose(ChoiceId::new(7)).unwrap(),
        SceneId::new(2)
    );
}

#[test]
fn invalid_choice_is_rejected() {
    let mut r = StoryRuntime::new(definition()).unwrap();
    r.enter_scene(SceneId::new(1)).unwrap();
    assert!(matches!(
        r.choose(ChoiceId::new(9)),
        Err(StoryError::InvalidTransition(_))
    ));
}

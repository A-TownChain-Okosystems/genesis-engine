use atc_genesis_story_ai::*;
#[test]
fn one_shot_trigger_fires_once() {
    let s=SceneId::new(1); let mut d=StoryDefinition::new(StoryId::new(1),"Triggers");
    d.scenes.insert(s,Scene{id:s,title:"S".into(),event:None});
    let mut r=StoryRuntime::new(d).unwrap();
    r.triggers.insert(Trigger{
        id:TriggerId::new(1),
        condition:TriggerCondition::Scene(1),
        consequences:vec![Consequence::AddCounter{key:"fires".into(),delta:1}],
        one_shot:true,
    }).unwrap();
    r.enter_scene(s).unwrap(); r.enter_scene(s).unwrap();
    assert_eq!(r.state.counters["fires"],1);
}
#[test]
fn failed_trigger_transaction_does_not_consume_one_shot() {
    let s=SceneId::new(1); let mut d=StoryDefinition::new(StoryId::new(1),"Triggers");
    d.scenes.insert(s,Scene{id:s,title:"S".into(),event:None});
    let mut r=StoryRuntime::new(d).unwrap();
    r.triggers.insert(Trigger{
        id:TriggerId::new(1),
        condition:TriggerCondition::Scene(1),
        consequences:vec![Consequence::Reputation{character:CharacterId::new(999),delta:1}],
        one_shot:true,
    }).unwrap();
    assert!(r.enter_scene(s).is_err());
    assert!(r.enter_scene(s).is_err());
}

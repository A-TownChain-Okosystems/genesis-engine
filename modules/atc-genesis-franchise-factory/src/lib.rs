//! Canonical Rust core for the Genesis Franchise Factory.

use sha2::{Digest, Sha256};

use std::collections::{BTreeMap, BTreeSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactKind { GameBible, WorldBible, Lore, Character, Creature, Quest, Level, Item, Weapon, Animation, Audio, Vfx, Combat, NpcAi, Economy, Multiplayer, Build, QaReport, LiveOpsPlan }

impl ArtifactKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GameBible => "game-bible", Self::WorldBible => "world-bible", Self::Lore => "lore",
            Self::Character => "character", Self::Creature => "creature", Self::Quest => "quest",
            Self::Level => "level", Self::Item => "item", Self::Weapon => "weapon", Self::Animation => "animation",
            Self::Audio => "audio", Self::Vfx => "vfx", Self::Combat => "combat", Self::NpcAi => "npc-ai",
            Self::Economy => "economy", Self::Multiplayer => "multiplayer", Self::Build => "build",
            Self::QaReport => "qa-report", Self::LiveOpsPlan => "liveops-plan",
        }
    }
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "game-bible"=>Self::GameBible, "world-bible"=>Self::WorldBible, "lore"=>Self::Lore,
            "character"=>Self::Character, "creature"=>Self::Creature, "quest"=>Self::Quest,
            "level"=>Self::Level, "item"=>Self::Item, "weapon"=>Self::Weapon, "animation"=>Self::Animation,
            "audio"=>Self::Audio, "vfx"=>Self::Vfx, "combat"=>Self::Combat, "npc-ai"=>Self::NpcAi,
            "economy"=>Self::Economy, "multiplayer"=>Self::Multiplayer, "build"=>Self::Build,
            "qa-report"=>Self::QaReport, "liveops-plan"=>Self::LiveOpsPlan, _=>return None
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRef { pub id: String, pub kind: ArtifactKind, pub version: String, pub producer: String, pub content_hash: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEnvelope {
    pub reference: ArtifactRef,
    pub dependencies: Vec<ArtifactRef>,
    pub evidence: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactContractError { EmptyId, EmptyVersion, EmptyProducer, EmptyDependencyId }

pub fn validate_artifact(artifact: &ArtifactEnvelope) -> Result<(), ArtifactContractError> {
    if artifact.reference.id.trim().is_empty() { return Err(ArtifactContractError::EmptyId); }
    if artifact.reference.version.trim().is_empty() { return Err(ArtifactContractError::EmptyVersion); }
    if artifact.reference.producer.trim().is_empty() { return Err(ArtifactContractError::EmptyProducer); }
    if artifact.dependencies.iter().any(|d| d.id.trim().is_empty()) { return Err(ArtifactContractError::EmptyDependencyId); }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameFactoryNode { pub id: &'static str, pub produces: ArtifactKind, pub requires: &'static [ArtifactKind] }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError { DuplicateNodeId, DuplicateProducer(ArtifactKind), MissingProducer { node: &'static str, kind: ArtifactKind }, DependencyCycle, MissingDependency { node: &'static str, kind: ArtifactKind } }

pub const GAME_FACTORY_GRAPH: &[GameFactoryNode] = &[
 GameFactoryNode{id:"concept",produces:ArtifactKind::GameBible,requires:&[]},
 GameFactoryNode{id:"world",produces:ArtifactKind::WorldBible,requires:&[ArtifactKind::GameBible]},
 GameFactoryNode{id:"lore",produces:ArtifactKind::Lore,requires:&[ArtifactKind::WorldBible]},
 GameFactoryNode{id:"character",produces:ArtifactKind::Character,requires:&[ArtifactKind::GameBible,ArtifactKind::Lore]},
 GameFactoryNode{id:"creature",produces:ArtifactKind::Creature,requires:&[ArtifactKind::WorldBible,ArtifactKind::Lore]},
 GameFactoryNode{id:"combat",produces:ArtifactKind::Combat,requires:&[ArtifactKind::GameBible,ArtifactKind::Character,ArtifactKind::Creature]},
 GameFactoryNode{id:"quest",produces:ArtifactKind::Quest,requires:&[ArtifactKind::WorldBible,ArtifactKind::Lore,ArtifactKind::Character]},
 GameFactoryNode{id:"level",produces:ArtifactKind::Level,requires:&[ArtifactKind::WorldBible,ArtifactKind::Quest,ArtifactKind::Creature]},
 GameFactoryNode{id:"item",produces:ArtifactKind::Item,requires:&[ArtifactKind::GameBible]},
 GameFactoryNode{id:"weapon",produces:ArtifactKind::Weapon,requires:&[ArtifactKind::Character,ArtifactKind::Combat]},
 GameFactoryNode{id:"animation",produces:ArtifactKind::Animation,requires:&[ArtifactKind::Character,ArtifactKind::Creature,ArtifactKind::Weapon]},
 GameFactoryNode{id:"audio",produces:ArtifactKind::Audio,requires:&[ArtifactKind::GameBible,ArtifactKind::Weapon]},
 GameFactoryNode{id:"vfx",produces:ArtifactKind::Vfx,requires:&[ArtifactKind::Combat,ArtifactKind::Weapon]},
 GameFactoryNode{id:"ai-npc",produces:ArtifactKind::NpcAi,requires:&[ArtifactKind::Character,ArtifactKind::Lore]},
 GameFactoryNode{id:"economy",produces:ArtifactKind::Economy,requires:&[ArtifactKind::GameBible,ArtifactKind::Item]},
 GameFactoryNode{id:"multiplayer",produces:ArtifactKind::Multiplayer,requires:&[ArtifactKind::GameBible,ArtifactKind::Combat,ArtifactKind::Economy]},
 GameFactoryNode{id:"build",produces:ArtifactKind::Build,requires:&[ArtifactKind::GameBible,ArtifactKind::WorldBible,ArtifactKind::Character,ArtifactKind::Quest,ArtifactKind::Level,ArtifactKind::Item,ArtifactKind::Weapon,ArtifactKind::Animation,ArtifactKind::Audio,ArtifactKind::Vfx,ArtifactKind::NpcAi,ArtifactKind::Multiplayer]},
 GameFactoryNode{id:"testing",produces:ArtifactKind::QaReport,requires:&[ArtifactKind::Build,ArtifactKind::Combat,ArtifactKind::Economy]},
 GameFactoryNode{id:"liveops",produces:ArtifactKind::LiveOpsPlan,requires:&[ArtifactKind::QaReport,ArtifactKind::Multiplayer,ArtifactKind::Economy]},
];

pub fn validate_graph(graph:&[GameFactoryNode])->Result<(),GraphError>{
 let mut ids=BTreeSet::new(); let mut producers=BTreeSet::new();
 for n in graph { if !ids.insert(n.id){return Err(GraphError::DuplicateNodeId)} if !producers.insert(n.produces){return Err(GraphError::DuplicateProducer(n.produces))} }
 for n in graph { for &r in n.requires { if !producers.contains(&r){return Err(GraphError::MissingProducer{node:n.id,kind:r})} } }
 Ok(())
}

pub fn topological_order(graph:&[GameFactoryNode])->Result<Vec<&'static str>,GraphError>{
 validate_graph(graph)?; let mut remaining:BTreeMap<ArtifactKind,&GameFactoryNode>=graph.iter().map(|n|(n.produces,n)).collect(); let mut done=BTreeSet::new(); let mut order=Vec::with_capacity(graph.len());
 while !remaining.is_empty(){ let mut ready:Vec<&GameFactoryNode>=remaining.values().copied().filter(|n|n.requires.iter().all(|r|done.contains(r))).collect(); if ready.is_empty(){return Err(GraphError::DependencyCycle)} ready.sort_by_key(|n|n.id); for n in ready {done.insert(n.produces);remaining.remove(&n.produces);order.push(n.id)} }
 Ok(order)
}

pub fn resolve_dependencies(node:&GameFactoryNode,artifacts:&[ArtifactRef])->Result<Vec<ArtifactRef>,GraphError>{
 node.requires.iter().map(|k|artifacts.iter().rev().find(|a|a.kind==*k).cloned().ok_or(GraphError::MissingDependency{node:node.id,kind:*k})).collect()
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub enum WorkflowStage { Input, Analyze, Plan, Produce, Quality, Integrate, Publish, Monitor, Optimize, Replicate }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum WorkflowError { EmptyStages, MustStartWithInput, MissingQualityGate, DuplicateStage }
pub fn validate_workflow(stages:&[WorkflowStage])->Result<(),WorkflowError>{
 if stages.is_empty(){return Err(WorkflowError::EmptyStages)} if stages[0]!=WorkflowStage::Input{return Err(WorkflowError::MustStartWithInput)}
 if !stages.contains(&WorkflowStage::Quality){return Err(WorkflowError::MissingQualityGate)} let mut seen=BTreeSet::new(); for s in stages{if !seen.insert(*s){return Err(WorkflowError::DuplicateStage)}} Ok(())
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub enum LifecyclePhase { Idea, Concept, Prototype, PreProd, Production, Alpha, Beta, Release, LiveOps, Expansion, Successor, Archived }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum LifecycleError { InvalidTransition{from:LifecyclePhase,to:LifecyclePhase} }
pub fn valid_transition(from:LifecyclePhase,to:LifecyclePhase)->bool{to==LifecyclePhase::Archived || (from as u8).checked_add(1).is_some_and(|n|n==to as u8)}
pub fn transition(from:LifecyclePhase,to:LifecyclePhase)->Result<LifecyclePhase,LifecycleError>{if valid_transition(from,to){Ok(to)}else{Err(LifecycleError::InvalidTransition{from,to})}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FranchiseStatus { Concept, InProduction, Testing, Live, Expanding, Archived }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStatus { Pending, InProgress, Complete, Skipped }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStage { pub name: String, pub factory: String, pub order: u32, pub enabled: bool, pub status: PipelineStatus }

pub fn default_pipeline() -> Vec<PipelineStage> {
    [
        ("Blueprint","ip_factory"),("World","world_factory"),("Character","character_factory"),
        ("Lore","lore_factory"),("Quest","quest_factory"),("Asset","ai_content_factory"),
        ("Game","world_factory"),("Testing","analytics_factory"),("LiveOps","liveops_factory"),
        ("Merchandise","merchandise_factory")
    ].into_iter().enumerate().map(|(i,(name,factory))| PipelineStage {
        name:name.into(), factory:factory.into(), order:i as u32, enabled:true, status:PipelineStatus::Pending
    }).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FranchiseBlueprint {
    pub name:String, pub genre:String, pub target_audience:String, pub world_count:u32,
    pub character_count:u32, pub quest_count:u32, pub economy_model:String,
    pub monetization:Vec<String>, pub platforms:Vec<String>
}
#[derive(Debug, Clone, PartialEq)]
pub struct Franchise {
    pub id:String, pub name:String, pub universe:String, pub status:FranchiseStatus,
    pub created_at:f64, pub blueprint:FranchiseBlueprint, pub factories_used:Vec<String>, pub progress:f64
}
pub struct FranchiseFactoryCore {
    pub version:String, pub initialized:bool, pub franchises:std::collections::BTreeMap<String,Franchise>,
    pub active_franchise:Option<String>, pub pipeline:Vec<PipelineStage>, pub pipeline_running:bool,
    pub events:Vec<String>, event_clock:f64
}
impl FranchiseFactoryCore {
    pub fn new(version:impl Into<String>)->Self {
        let version=version.into();
        Self { version:version.clone(), initialized:true, franchises:std::collections::BTreeMap::new(),
            active_franchise:None, pipeline:default_pipeline(), pipeline_running:false,
            events:vec![format!("GFFInitialized:{}",version)], event_clock:1.0 }
    }
    pub fn create_franchise(&mut self, blueprint:FranchiseBlueprint, now:Option<f64>)->Result<String,&'static str> {
        if blueprint.name.is_empty(){return Err("blueprint.name darf nicht leer sein");}
        let ts=now.unwrap_or(self.event_clock);
        let id={ let mut h=sha2::Sha256::new(); h.update(format!("{}|{}",blueprint.name,ts).as_bytes()); h.finalize().iter().take(8).map(|b| format!("{:02x}",b)).collect::<String>() };
        self.franchises.insert(id.clone(),Franchise{id:id.clone(),name:blueprint.name.clone(),
            universe:format!("{} Universe",blueprint.name),status:FranchiseStatus::Concept,created_at:ts,
            blueprint,factories_used:Vec::new(),progress:0.0});
        self.active_franchise=Some(id.clone()); self.events.push(format!("FranchiseCreated:{}:{}",id,self.franchises[&id].name));
        self.event_clock+=1.0; Ok(id)
    }
    pub fn reset_pipeline(&mut self){for s in &mut self.pipeline{s.status=PipelineStatus::Pending;}}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageResult {
    pub success: bool,
    pub note: String,
}
impl StageResult {
    pub fn dry_run() -> Self { Self { success: true, note: "dry-run: kein Executor injiziert".into() } }
}

pub type StageExecutor = fn(&PipelineStage, &Franchise) -> StageResult;

fn noop_executor(_stage: &PipelineStage, _franchise: &Franchise) -> StageResult {
    StageResult::dry_run()
}

impl FranchiseFactoryCore {
    pub fn run_pipeline(&mut self, franchise_id: &str, executor: Option<StageExecutor>) -> Result<(), String> {
        if !self.franchises.contains_key(franchise_id) {
            return Err(format!("unknown franchise: {}", franchise_id));
        }
        if self.pipeline_running {
            return Err("pipeline already running".into());
        }
        self.pipeline_running = true;
        self.events.push(format!("PipelineStarted:{}", franchise_id));
        let result = (|| {
            let mut stages = self.pipeline.clone();
            stages.sort_by_key(|s| s.order);
            let total = stages.len();
            let mut done = 0usize;
            for stage in stages {
                if !stage.enabled {
                    if let Some(current) = self.pipeline.iter_mut().find(|s| s.order == stage.order) {
                        current.status = PipelineStatus::Skipped;
                    }
                    done += 1;
                    continue;
                }
                if let Some(current) = self.pipeline.iter_mut().find(|s| s.order == stage.order) {
                    current.status = PipelineStatus::InProgress;
                }
                let franchise = self.franchises.get(franchise_id).cloned().expect("validated above");
                let outcome = executor.unwrap_or(noop_executor)(&stage, &franchise);
                if !outcome.success {
                    if let Some(current) = self.pipeline.iter_mut().find(|s| s.order == stage.order) {
                        current.status = PipelineStatus::Pending;
                    }
                    self.events.push(format!("StageFailed:{}:{}:{}", franchise_id, stage.name, outcome.note));
                    return Err(format!("pipeline stage failed: {}", stage.name));
                }
                if let Some(current) = self.pipeline.iter_mut().find(|s| s.order == stage.order) {
                    current.status = PipelineStatus::Complete;
                }
                if let Some(franchise) = self.franchises.get_mut(franchise_id) {
                    if !franchise.factories_used.contains(&stage.factory) {
                        franchise.factories_used.push(stage.factory.clone());
                    }
                    done += 1;
                    franchise.progress = ((done as f64 / total.max(1) as f64) * 10000.0).round() / 10000.0;
                }
                self.events.push(format!("StageComplete:{}:{}", franchise_id, stage.name));
            }
            if let Some(franchise) = self.franchises.get_mut(franchise_id) {
                franchise.progress = 1.0;
            }
            self.events.push(format!("PipelineComplete:{}", franchise_id));
            Ok(())
        })();
        self.pipeline_running = false;
        result
    }
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn graph_validates_and_orders(){validate_graph(GAME_FACTORY_GRAPH).unwrap();let o=topological_order(GAME_FACTORY_GRAPH).unwrap();assert_eq!(o.first(),Some(&"concept"));assert_eq!(o.last(),Some(&"liveops"));assert_eq!(o.len(),GAME_FACTORY_GRAPH.len());
        assert_eq!(ArtifactKind::GameBible.as_str(), "game-bible");
        assert_eq!(ArtifactKind::parse("qa-report"), Some(ArtifactKind::QaReport));
        assert_eq!(ArtifactKind::parse("unknown"), None);}
 #[test]fn graph_fails_closed(){let g=[GameFactoryNode{id:"broken",produces:ArtifactKind::Build,requires:&[ArtifactKind::Lore]}];assert!(matches!(validate_graph(&g),Err(GraphError::MissingProducer{..})));}
 #[test]fn workflow_requires_quality(){assert_eq!(validate_workflow(&[WorkflowStage::Input,WorkflowStage::Produce]),Err(WorkflowError::MissingQualityGate));
        assert!(validate_workflow(&[WorkflowStage::Input,WorkflowStage::Analyze,WorkflowStage::Plan,WorkflowStage::Produce,WorkflowStage::Quality,WorkflowStage::Integrate,WorkflowStage::Publish,WorkflowStage::Monitor,WorkflowStage::Optimize,WorkflowStage::Replicate]).is_ok());
        let duplicate_kind = [
            ArtifactRef{id:"old".into(),kind:ArtifactKind::Item,version:"1.0.0".into(),producer:"a".into(),content_hash:None},
            ArtifactRef{id:"new".into(),kind:ArtifactKind::Item,version:"1.0.0".into(),producer:"b".into(),content_hash:None},
        ];
        let item_node = &GAME_FACTORY_GRAPH[8];
        assert_eq!(resolve_dependencies(item_node, &duplicate_kind).unwrap()[0].id, "new");
        assert_eq!(validate_workflow(&[WorkflowStage::Input,WorkflowStage::Quality,WorkflowStage::Quality]),Err(WorkflowError::DuplicateStage));}
 #[test]fn lifecycle_is_sequential_or_archive(){assert!(transition(LifecyclePhase::Idea,LifecyclePhase::Concept).is_ok());assert!(transition(LifecyclePhase::Idea,LifecyclePhase::Production).is_err());assert!(transition(LifecyclePhase::Idea,LifecyclePhase::Archived).is_ok());
        let a=ArtifactEnvelope{reference:ArtifactRef{id:"x".into(),kind:ArtifactKind::Item,version:"1.0.0".into(),producer:"factory".into(),content_hash:None},dependencies:vec![],evidence:vec![]};
        validate_artifact(&a).unwrap();}
}

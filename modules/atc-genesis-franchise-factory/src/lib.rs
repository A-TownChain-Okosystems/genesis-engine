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

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum FactoryId { Text, Software, Game, Marketing, Business, Document, Research, Ecommerce, CustomerService, Automation, Knowledge, Agent, Franchise, Startup, Education, Book, VirtualWorld }
impl FactoryId {
 pub const fn as_str(self)->&'static str{match self{
  Self::Text=>"text-content",Self::Software=>"software",Self::Game=>"game",Self::Marketing=>"marketing",Self::Business=>"business",Self::Document=>"document",Self::Research=>"research",Self::Ecommerce=>"e-commerce",Self::CustomerService=>"customer-service",Self::Automation=>"automation",Self::Knowledge=>"knowledge",Self::Agent=>"agent",Self::Franchise=>"franchise",Self::Startup=>"startup",Self::Education=>"education",Self::Book=>"book",Self::VirtualWorld=>"virtual-world"}}
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct WorkflowDefinition { pub id:String,pub name:String,pub stages:Vec<WorkflowStage>,pub capabilities:Vec<String>,pub outputs:Vec<String>,pub description:String }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum WorkflowRegistryError { EmptyRegistry,MustStartWithInput,MissingQualityGate,DuplicateStage,UnknownWorkflow }
pub struct WorkflowRegistry { definitions:BTreeMap<String,WorkflowDefinition> }
impl WorkflowRegistry {
 pub fn new(definitions:Vec<WorkflowDefinition>)->Result<Self,WorkflowRegistryError>{
  if definitions.is_empty(){return Err(WorkflowRegistryError::EmptyRegistry)}
  let mut map=BTreeMap::new();
  for d in definitions {
   validate_workflow(&d.stages).map_err(|e|match e{WorkflowError::EmptyStages=>WorkflowRegistryError::EmptyRegistry,WorkflowError::MustStartWithInput=>WorkflowRegistryError::MustStartWithInput,WorkflowError::MissingQualityGate=>WorkflowRegistryError::MissingQualityGate,WorkflowError::DuplicateStage=>WorkflowRegistryError::DuplicateStage})?;
   map.insert(d.id.clone(),d);
  }
  Ok(Self{definitions:map})
 }
 pub fn get(&self,id:&str)->Result<&WorkflowDefinition,WorkflowRegistryError>{self.definitions.get(id).ok_or(WorkflowRegistryError::UnknownWorkflow)}
 pub fn all(&self)->impl Iterator<Item=&WorkflowDefinition>{self.definitions.values()}
}
fn default_workflow(factory:FactoryId,name:&str,c:&[&str],o:&[&str],description:&str)->WorkflowDefinition{
 WorkflowDefinition{id:factory.as_str().into(),name:name.into(),
 stages:vec![WorkflowStage::Input,WorkflowStage::Analyze,WorkflowStage::Plan,WorkflowStage::Produce,WorkflowStage::Quality,WorkflowStage::Integrate,WorkflowStage::Publish,WorkflowStage::Monitor,WorkflowStage::Optimize],
 capabilities:c.iter().map(|x|(*x).into()).collect(),outputs:o.iter().map(|x|(*x).into()).collect(),description:description.into()}
}
pub fn default_workflows()->Vec<WorkflowDefinition>{vec![
 default_workflow(FactoryId::Text,"Text / Content Factory",&["writing","translation","seo"],&["content","localized-content"],"Text and content production."),
 default_workflow(FactoryId::Software,"Software Factory",&["specification","architecture","code","tests","deployment"],&["source","tests","documentation","release"],"Software delivery from idea to deployment."),
 default_workflow(FactoryId::Game,"Game Factory",&["game-design","world","lore","characters","quests","gameplay","assets","ai-npc","testing","liveops"],&["game-bible","game-content","build","liveops-plan"],"End-to-end game production."),
 default_workflow(FactoryId::Marketing,"Marketing Factory",&["campaigns","ads","landing-pages","experiments","analytics"],&["campaign","creative-plan","report"],"Campaign planning and optimization."),
 default_workflow(FactoryId::Business,"Business Factory",&["business-model","market-analysis","financial-model","pricing","kpis"],&["business-plan","financial-model","kpi-plan"],"Business model production."),
 default_workflow(FactoryId::Document,"Document Factory",&["contracts","offers","reports","sops","manuals"],&["documents","document-set"],"Controlled business document production."),
 default_workflow(FactoryId::Research,"Research Factory",&["research","source-analysis","monitoring","synthesis"],&["research-report","knowledge-update"],"Evidence-oriented research workflows."),
 default_workflow(FactoryId::Ecommerce,"E-Commerce Factory",&["catalog","product-analysis","pricing","sales-analysis"],&["catalog","product-content","sales-report"],"E-commerce content and operations."),
 default_workflow(FactoryId::CustomerService,"Customer-Service Factory",&["support","faq","triage","crm","escalation"],&["resolution","faq-update","support-report"],"Customer support automation."),
 default_workflow(FactoryId::Automation,"Automation Factory",&["triggers","agents","apis","data","notifications"],&["automation","execution-log"],"Event-driven multi-step automation."),
 default_workflow(FactoryId::Knowledge,"Knowledge Factory",&["ingestion","rag","semantic-search","knowledge-graph","memory"],&["knowledge-base","index","graph"],"Document-to-knowledge transformation."),
 default_workflow(FactoryId::Agent,"Agent Factory",&["identity","tools","memory","permissions","workflows","agent-messaging"],&["agent-definition","policy","workflow"],"Controlled AI-agent creation."),
 default_workflow(FactoryId::Franchise,"Franchise Factory",&["business-model","brand","product","content","software","marketing","sales","automation","replication"],&["franchise-package","operating-model","replication-plan"],"Replicate a validated business system as a franchise package."),
 default_workflow(FactoryId::Startup,"Startup Factory",&["validation","mvp","branding","product","launch","kpis"],&["startup-package","mvp-plan","launch-plan"],"Startup formation from idea to launch."),
 default_workflow(FactoryId::Education,"Education Factory",&["curriculum","learning-material","exercises","assessment","tutoring"],&["course","learning-platform-plan","assessment"],"Education product production."),
 default_workflow(FactoryId::Book,"Book Factory",&["research","outline","writing","editing","layout","translation","publishing"],&["manuscript","book-package","publication-plan"],"Book production and publication."),
 default_workflow(FactoryId::VirtualWorld,"Virtual World Factory",&["world","geography","cities","buildings","npcs","economy","factions","lore","simulation"],&["world-bible","world-data","simulation"],"Persistent virtual-world production."),
]}
/// Canonical Game Factory subsystem capability matrix, matching the reference contract.
pub fn game_subsystems() -> BTreeMap<&'static str, &'static [&'static str]> {
 let mut m=BTreeMap::new();
 m.insert("concept",&["genre","target","platforms","usp","core-loop","modes","monetization","technical-requirements"][..]);
 m.insert("world",&["continents","regions","biomes","cities","dungeons","buildings","climate","day-night","weather","portals"][..]);
 m.insert("lore",&["origin","peoples","factions","wars","timeline","secrets","canon","artifacts"][..]);
 m.insert("character",&["player","npc","classes","attributes","skills","progression","relationships"][..]);
 m.insert("creature",&["anatomy","abilities","weaknesses","attacks","movement","loot","variants","boss-mechanics"][..]);
 m.insert("combat",&["melee","ranged","magic","combos","dodge","parry","status","boss-phases","pvp-balance"][..]);
 m.insert("quest",&["main","side","faction","events","puzzles","boss","hidden","dynamic"][..]);
 m.insert("level",&["terrain","rooms","paths","encounters","loot","checkpoints","puzzles","secrets","scaling"][..]);
 m.insert("item",&["weapons","armor","accessories","consumables","resources","relics","artifacts","skins","crafting"][..]);
 m.insert("weapon",&["concept","design","3d","animation","vfx","sound","gameplay"][..]);
 m.insert("animation",&["idle","walk","run","jump","attack","combo","hit","death","emotes","interactions"][..]);
 m.insert("ai-npc",&["identity","memory","personality","goals","knowledge","behavior","routine","relationships"][..]);
 m.insert("audio",&["sfx","voice","ambient","music","dynamic-music","spatial-audio"][..]);
 m.insert("vfx",&["fire","water","explosions","magic","energy","portals","weather","abilities","boss-effects"][..]);
 m.insert("economy",&["loot","resources","crafting","pricing","demand","inflation","progression","rewards"][..]);
 m.insert("multiplayer",&["matchmaking","lobby","party","guilds","pvp","pve","raids","leaderboards","seasons","server"][..]);
 m.insert("testing",&["bugs","exploits","levels","combat","quests","economy","performance","network","ui","progression","ai-playtests"][..]);
 m.insert("liveops",&["telemetry","retention","funnel","balance","bugs","seasons","events","content"][..]);
 m
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub enum LifecyclePhase { Idea, Concept, Prototype, PreProd, Production, Alpha, Beta, Release, LiveOps, Expansion, Successor, Archived }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum LifecycleError { InvalidTransition{from:LifecyclePhase,to:LifecyclePhase} }
pub fn valid_transition(from:LifecyclePhase,to:LifecyclePhase)->bool{to==LifecyclePhase::Archived || (from as u8).checked_add(1).is_some_and(|n|n==to as u8)}
pub fn transition(from:LifecyclePhase,to:LifecyclePhase)->Result<LifecyclePhase,LifecycleError>{if valid_transition(from,to){Ok(to)}else{Err(LifecycleError::InvalidTransition{from,to})}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneStatus { NotStarted, InProgress, Achieved }

#[derive(Debug, Clone, PartialEq)]
pub struct Kpi {
    pub dau: u64, pub mau: u64, pub revenue: f64, pub retention: f64,
    pub crash_free: f64, pub rating: f64, pub sentiment: f64,
}
impl Default for Kpi {
    fn default() -> Self {
        Self { dau: 0, mau: 0, revenue: 0.0, retention: 0.0, crash_free: 100.0, rating: 0.0, sentiment: 0.5 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Milestone {
    pub id: String, pub franchise_id: String, pub name: String, pub phase: LifecyclePhase,
    pub target: u64, pub achieved: u64, pub status: MilestoneStatus,
    pub criteria: Vec<String>, pub done: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhaseTemplate {
    pub phase: LifecyclePhase, pub name: String, pub desc: String,
    pub duration_days: u32, pub deliverables: Vec<String>, pub criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhaseHistory {
    pub phase: LifecyclePhase, pub entered: f64, pub exited: f64,
    pub notes: String, pub success: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LifecycleFranchise {
    pub id: String, pub name: String, pub phase: LifecyclePhase,
    pub history: Vec<PhaseHistory>, pub start: f64, pub target: f64,
    pub budget: u64, pub spent: u64, pub team: Vec<String>, pub risk: u8,
    pub prob: f64, pub milestones: Vec<String>, pub kpi: Kpi,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LifecycleTransition {
    pub id: String, pub franchise_id: String, pub from: LifecyclePhase,
    pub to: LifecyclePhase, pub timestamp: f64, pub by: String, pub notes: String,
}

fn lifecycle_id(input: &str) -> String {
    let mut h = Sha256::new();
    h.update(input.as_bytes());
    h.finalize().iter().take(8).map(|b| format!("{:02x}", b)).collect()
}

fn init_phase_templates() -> BTreeMap<LifecyclePhase, PhaseTemplate> {
    let rows = [
        (LifecyclePhase::Idea, "Idea", "Concept", 14, vec!["Vision"], vec!["Approved"]),
        (LifecyclePhase::Concept, "Design", "Design", 30, vec!["GDD"], vec!["GDD OK"]),
        (LifecyclePhase::Prototype, "Playable", "Playable", 60, vec!["Slice"], vec!["Playable"]),
        (LifecyclePhase::PreProd, "Planning", "Planning", 90, vec!["Plan"], vec!["Plan OK"]),
        (LifecyclePhase::Production, "Content", "Content", 365, vec!["Levels"], vec!["Complete"]),
        (LifecyclePhase::Alpha, "Features", "Features", 60, vec!["Features"], vec!["Alpha"]),
        (LifecyclePhase::Beta, "Polish", "Polish", 60, vec!["Bugs"], vec!["Beta"]),
        (LifecyclePhase::Release, "Launch", "Launch", 30, vec!["Gold"], vec!["Launched"]),
        (LifecyclePhase::LiveOps, "Ops", "Ops", 0, vec!["Seasons"], vec!["Active"]),
        (LifecyclePhase::Expansion, "DLC", "DLC", 180, vec!["DLC"], vec!["DLC OK"]),
        (LifecyclePhase::Successor, "Next", "Next", 365, vec!["Plan"], vec!["New"]),
        (LifecyclePhase::Archived, "End", "End", 0, Vec::<&str>::new(), vec!["Closed"]),
    ];
    rows.into_iter().map(|(phase,name,desc,duration,deliverables,criteria)| (
        phase,
        PhaseTemplate {
            phase, name:name.into(), desc:desc.into(), duration_days:duration,
            deliverables:deliverables.into_iter().map(str::to_owned).collect(),
            criteria:criteria.into_iter().map(str::to_owned).collect(),
        }
    )).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleManagerError {
    EmptyName,
    UnknownFranchise,
    InvalidTransition { from: LifecyclePhase, to: LifecyclePhase },
    UnknownMilestone,
}

pub struct LifecycleManager {
    pub franchises: BTreeMap<String, LifecycleFranchise>,
    pub templates: BTreeMap<LifecyclePhase, PhaseTemplate>,
    pub transitions: Vec<LifecycleTransition>,
    pub milestones: BTreeMap<String, Milestone>,
    now: f64,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self { franchises:BTreeMap::new(), templates:init_phase_templates(), transitions:Vec::new(), milestones:BTreeMap::new(), now:0.0 }
    }

    pub fn register(&mut self, name: impl Into<String>, budget:u64, target:f64, now:Option<f64>) -> Result<String, LifecycleManagerError> {
        let name=name.into();
        if name.is_empty() { return Err(LifecycleManagerError::EmptyName); }
        let ts=now.unwrap_or(self.now);
        let id=lifecycle_id(&format!("{}|{}", name, ts));
        let history=vec![PhaseHistory{phase:LifecyclePhase::Idea,entered:ts,exited:0.0,notes:"Created".into(),success:false}];
        self.franchises.insert(id.clone(), LifecycleFranchise {
            id:id.clone(), name, phase:LifecyclePhase::Idea, history, start:ts,
            target, budget, spent:0, team:Vec::new(), risk:5, prob:0.5,
            milestones:Vec::new(), kpi:Kpi::default()
        });
        Ok(id)
    }

    pub fn transition(&mut self, franchise_id:&str, new_phase:LifecyclePhase, by:impl Into<String>, notes:impl Into<String>, now:Option<f64>) -> Result<(), LifecycleManagerError> {
        let ts=now.unwrap_or(self.now);
        let franchise=self.franchises.get_mut(franchise_id).ok_or(LifecycleManagerError::UnknownFranchise)?;
        let old=franchise.phase;
        if !valid_transition(old,new_phase) { return Err(LifecycleManagerError::InvalidTransition{from:old,to:new_phase}); }
        if let Some(previous)=franchise.history.last_mut() { previous.exited=ts; previous.success=true; }
        franchise.phase=new_phase;
        franchise.history.push(PhaseHistory{phase:new_phase,entered:ts,exited:0.0,notes:notes.into(),success:false});
        let transition_id=lifecycle_id(&format!("{}|{}", franchise_id, ts));
        self.transitions.push(LifecycleTransition{id:transition_id,franchise_id:franchise_id.into(),from:old,to:new_phase,timestamp:ts,by:by.into(),notes:franchise.history.last().map(|h|h.notes.clone()).unwrap_or_default()});
        Ok(())
    }

    pub fn add_milestone(&mut self, franchise_id:&str, name:impl Into<String>, phase:LifecyclePhase, target:u64, criteria:Vec<String>) -> Result<String, LifecycleManagerError> {
        if !self.franchises.contains_key(franchise_id) { return Err(LifecycleManagerError::UnknownFranchise); }
        let name=name.into();
        let id=lifecycle_id(&format!("{}|{}", name, self.now));
        let milestone=Milestone{id:id.clone(),franchise_id:franchise_id.into(),name,phase,target,achieved:0,status:MilestoneStatus::NotStarted,criteria,done:Vec::new()};
        self.milestones.insert(id.clone(),milestone.clone());
        self.franchises.get_mut(franchise_id).unwrap().milestones.push(id.clone());
        Ok(id)
    }

    pub fn achieve_milestone(&mut self, milestone_id:&str, achieved:u64, done:Vec<String>) -> Result<(), LifecycleManagerError> {
        let milestone=self.milestones.get_mut(milestone_id).ok_or(LifecycleManagerError::UnknownMilestone)?;
        milestone.achieved=achieved;
        milestone.done=done;
        milestone.status=if achieved >= milestone.target { MilestoneStatus::Achieved } else { MilestoneStatus::InProgress };
        if let Some(franchise)=self.franchises.get_mut(&milestone.franchise_id) {
            if let Some(copy)=franchise.milestones.iter_mut().find(|m|m.id==milestone_id) { *copy=milestone.clone(); }
        }
        Ok(())
    }

    pub fn health(&self, franchise_id:&str) -> Result<(LifecyclePhase,u8,f64,f64,u64,f64), LifecycleManagerError> {
        let f=self.franchises.get(franchise_id).ok_or(LifecycleManagerError::UnknownFranchise)?;
        let budget_used=if f.budget==0 {0.0} else { ((f.spent as f64 / f.budget as f64)*10000.0).round()/10000.0 };
        Ok((f.phase,f.risk,f.prob,budget_used,f.kpi.dau,f.kpi.crash_free))
    }
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GffEvent {
    pub event: String,
    pub ts: u64,
    pub fid: Option<String>,
    pub name: Option<String>,
    pub stage: Option<String>,
    pub note: Option<String>,
    pub progress: Option<u32>,
}

pub struct FranchiseFactoryCore {
    pub version:String, pub initialized:bool, pub franchises:std::collections::BTreeMap<String,Franchise>,
    pub active_franchise:Option<String>, pub pipeline:Vec<PipelineStage>, pub pipeline_running:bool,
    pub events:Vec<GffEvent>, event_clock:u64
}
impl FranchiseFactoryCore {
    pub fn new(version:impl Into<String>)->Self {
        let version=version.into();
        Self { version:version.clone(), initialized:true, franchises:std::collections::BTreeMap::new(),
            active_franchise:None, pipeline:default_pipeline(), pipeline_running:false,
            events:vec![GffEvent{event:"GFFInitialized".into(),ts:0,fid:None,name:None,stage:None,note:None,progress:None}], event_clock:1 }
    }
    pub fn create_franchise(&mut self, blueprint:FranchiseBlueprint, now:Option<f64>)->Result<String,&'static str> {
        if blueprint.name.is_empty(){return Err("blueprint.name darf nicht leer sein");}
        let ts=now.unwrap_or(self.event_clock as f64);
        let id={ let mut h=sha2::Sha256::new(); h.update(format!("{}|{}",blueprint.name,ts).as_bytes()); h.finalize().iter().take(8).map(|b| format!("{:02x}",b)).collect::<String>() };
        self.franchises.insert(id.clone(),Franchise{id:id.clone(),name:blueprint.name.clone(),
            universe:format!("{} Universe",blueprint.name),status:FranchiseStatus::Concept,created_at:ts,
            blueprint,factories_used:Vec::new(),progress:0.0});
        self.active_franchise=Some(id.clone());
        self.events.push(GffEvent{event:"FranchiseCreated".into(),ts:self.event_clock,fid:Some(id.clone()),name:Some(blueprint.name.clone()),stage:None,note:None,progress:None});
        self.event_clock+=1; Ok(id)
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
        if !self.franchises.contains_key(franchise_id) { return Err(format!("Franchise {} nicht gefunden", franchise_id)); }
        if self.pipeline_running { return Err("Pipeline laeuft bereits (pipeline_running)".into()); }
        self.pipeline_running = true;
        self.events.push(GffEvent{event:"PipelineStarted".into(),ts:self.event_clock,fid:Some(franchise_id.into()),name:None,stage:None,note:None,progress:None});
        self.event_clock += 1;
        let result = (|| {
            let total = self.pipeline.iter().filter(|s| s.enabled).count();
            let mut done = 0usize;
            let mut stages = self.pipeline.clone();
            stages.sort_by_key(|s| s.order);
            for stage in stages {
                if !stage.enabled {
                    if let Some(current)=self.pipeline.iter_mut().find(|s| s.order==stage.order) { current.status=PipelineStatus::Skipped; }
                    continue;
                }
                if let Some(current)=self.pipeline.iter_mut().find(|s| s.order==stage.order) { current.status=PipelineStatus::InProgress; }
                let franchise=self.franchises.get(franchise_id).cloned().expect("validated above");
                let outcome=executor.unwrap_or(noop_executor)(&stage,&franchise);
                if !outcome.success {
                    self.events.push(GffEvent{event:"StageFailed".into(),ts:self.event_clock,fid:Some(franchise_id.into()),name:None,stage:Some(stage.name.clone()),note:Some(outcome.note),progress:None});
                    self.event_clock += 1;
                    return Err(format!("Stage {} fehlgeschlagen",stage.name));
                }
                if let Some(current)=self.pipeline.iter_mut().find(|s| s.order==stage.order) { current.status=PipelineStatus::Complete; }
                if let Some(franchise)=self.franchises.get_mut(franchise_id) {
                    if !franchise.factories_used.contains(&stage.factory) { franchise.factories_used.push(stage.factory.clone()); }
                    done += 1;
                    franchise.progress=if total==0 {0.0} else {((done as f64/total as f64)*10000.0).round()/10000.0};
                    let progress=(franchise.progress*10000.0) as u32;
                    self.events.push(GffEvent{event:"StageComplete".into(),ts:self.event_clock,fid:Some(franchise_id.into()),name:None,stage:Some(stage.name.clone()),note:None,progress:Some(progress)});
                }
                self.event_clock += 1;
            }
            let progress=self.franchises.get(franchise_id).map(|f|f.progress).unwrap_or(0.0);
            self.events.push(GffEvent{event:"PipelineComplete".into(),ts:self.event_clock,fid:Some(franchise_id.into()),name:None,stage:None,note:None,progress:Some((progress*10000.0) as u32)});
            self.event_clock += 1;
            Ok(())
        })();
        self.pipeline_running=false;
        result
    }
}


#[cfg(test)]
mod workflow_conformance_tests {
 use super::*;
 #[test] fn default_workflows_are_valid() {
  let defs=default_workflows();
  assert_eq!(defs.len(),6);
  let registry=WorkflowRegistry::new(defs).unwrap();
  assert_eq!(registry.get("franchise").unwrap().outputs,vec!["franchise-package","operating-model","replication-plan"]);
 }
 #[test] fn registry_rejects_invalid_workflows() {
  let d=WorkflowDefinition{id:"bad".into(),name:"bad".into(),stages:vec![WorkflowStage::Analyze,WorkflowStage::Quality],capabilities:vec![],outputs:vec![],description:String::new()};
  assert_eq!(WorkflowRegistry::new(vec![d]),Err(WorkflowRegistryError::MustStartWithInput));
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
 
 #[test]
 fn franchise_pipeline_is_deterministic_and_fail_closed() {
     let mut core = FranchiseFactoryCore::new("1.0.0");
     let blueprint = FranchiseBlueprint {
         name:"Genesis".into(), genre:"Action".into(), target_audience:"General".into(),
         world_count:1, character_count:2, quest_count:3, economy_model:"fixed".into(),
         monetization:vec!["premium".into()], platforms:vec!["pc".into()]
     };
     let id = core.create_franchise(blueprint, Some(0.0)).unwrap();
     assert_eq!(id, "ff1d7be7c852f459".to_string());
     core.pipeline[1].enabled = false;
     core.run_pipeline(&id, None).unwrap();
     assert!(core.pipeline.iter().any(|s| s.status == PipelineStatus::Skipped));
     assert_eq!(core.franchises[&id].progress, 1.0);
     assert_eq!(core.franchises[&id].factories_used.len(), 9);
     assert_eq!(core.events[0].event, "GFFInitialized");
     assert_eq!(core.events[0].ts, 0);
     assert_eq!(core.events[1].event, "FranchiseCreated");
     assert_eq!(core.events[1].ts, 1);
     assert!(core.events.iter().any(|e| e.event == "PipelineComplete" && e.progress == Some(10000)));
     core.reset_pipeline();
     assert!(core.pipeline.iter().all(|s| s.status == PipelineStatus::Pending));
 }

 fn failing_executor(stage:&PipelineStage, _franchise:&Franchise) -> StageResult {
     if stage.name == "Quest" { StageResult { success:false, note:"forced failure".into() } }
     else { StageResult::dry_run() }
 }

 #[test]
 fn franchise_pipeline_failure_resets_running_state() {
     let mut core = FranchiseFactoryCore::new("1.0.0");
     let id = core.create_franchise(FranchiseBlueprint {
         name:"Failure".into(), genre:"Test".into(), target_audience:"Test".into(),
         world_count:1, character_count:0, quest_count:0, economy_model:"".into(),
         monetization:vec![], platforms:vec![]
     }, Some(0.0)).unwrap();
     let err = core.run_pipeline(&id, Some(failing_executor)).unwrap_err();
     assert!(err.contains("Quest"));
     assert!(!core.pipeline_running);
     assert!(core.events.iter().any(|e| e.event == "StageFailed"));
 }

 #[test]
 fn lifecycle_manager_matches_reference_defaults() {
     let mut manager = LifecycleManager::new();
     assert_eq!(manager.templates.len(), 12);
     let id = manager.register("Genesis", 100, 10.0, Some(0.0)).unwrap();
     assert_eq!(id, "ff1d7be7c852f459");
     assert!(manager.transition(&id, LifecyclePhase::Concept, "system", "", Some(1.0)).is_ok());
     assert!(matches!(
         manager.transition(&id, LifecyclePhase::Production, "system", "", Some(2.0)),
         Err(LifecycleManagerError::InvalidTransition{..})
     ));
     let mid = manager.add_milestone(&id, "Slice", LifecyclePhase::Prototype, 2, vec!["Playable".into()]).unwrap();
     manager.achieve_milestone(&mid, 2, vec!["Playable".into()]).unwrap();
     assert_eq!(manager.milestones[&mid].status, MilestoneStatus::Achieved);
     assert_eq!(manager.franchises[&id].milestones, vec![mid.clone()]);
     let health=manager.health(&id).unwrap();
     assert_eq!(health.3, 0.0);
 }
 #[test]fn lifecycle_is_sequential_or_archive(){assert!(transition(LifecyclePhase::Idea,LifecyclePhase::Concept).is_ok());assert!(transition(LifecyclePhase::Idea,LifecyclePhase::Production).is_err());assert!(transition(LifecyclePhase::Idea,LifecyclePhase::Archived).is_ok());
        let a=ArtifactEnvelope{reference:ArtifactRef{id:"x".into(),kind:ArtifactKind::Item,version:"1.0.0".into(),producer:"factory".into(),content_hash:None},dependencies:vec![],evidence:vec![]};
        validate_artifact(&a).unwrap();}
}

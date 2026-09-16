use std::collections::BTreeMap;
use std::fmt;

pub mod jobs;
pub mod memory;
pub mod spatial;
pub use jobs::{JobError, JobId, JobQueue};
pub use memory::{Allocation, GenerationalStorage, LinearAllocator};
pub use spatial::{Aabb, Ray, Sphere, SpatialError, SpatialWorld};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GenerationalId { pub index: u32, pub generation: u32 }
impl GenerationalId { pub const INVALID: Self = Self { index: u32::MAX, generation: 0 }; pub const fn new(index:u32,generation:u32)->Self{Self{index,generation}} }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeStep { pub frame:u64, pub delta_seconds:f32, pub elapsed_seconds:f64 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeState { frame:u64, elapsed_seconds:f64 }
impl Default for TimeState { fn default()->Self{Self{frame:0,elapsed_seconds:0.0}} }
impl TimeState { pub fn advance(&mut self,delta:f32)->Result<TimeStep,TimeError>{if !delta.is_finite(){return Err(TimeError::NonFiniteDelta)} if delta<0.0{return Err(TimeError::NegativeDelta)} let frame=self.frame;self.frame=self.frame.checked_add(1).ok_or(TimeError::FrameOverflow)?;self.elapsed_seconds+=delta as f64;Ok(TimeStep{frame,delta_seconds:delta,elapsed_seconds:self.elapsed_seconds})} pub const fn frame(&self)->u64{self.frame} pub const fn elapsed_seconds(&self)->f64{self.elapsed_seconds} }
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum TimeError{NonFiniteDelta,NegativeDelta,FrameOverflow}
#[derive(Clone,Debug,PartialEq,Eq)] pub struct Event { pub topic:String,pub payload:Vec<u8> }
#[derive(Default)] pub struct EventBus { events:Vec<Event> }
impl EventBus { pub fn publish(&mut self,event:Event)->Result<(),EventError>{if event.topic.is_empty(){return Err(EventError::EmptyTopic)}self.events.push(event);Ok(())} pub fn drain(&mut self)->Vec<Event>{std::mem::take(&mut self.events)} pub fn len(&self)->usize{self.events.len()} }
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum EventError{EmptyTopic}
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash,PartialOrd,Ord)] pub struct ModuleId(pub u128);
#[derive(Clone,Debug,PartialEq,Eq)] pub struct ModuleDescriptor { pub id:ModuleId,pub name:String,pub version:String }
#[derive(Default)] pub struct ModuleRegistry { modules:BTreeMap<ModuleId,ModuleDescriptor> }
impl ModuleRegistry { pub fn register(&mut self,module:ModuleDescriptor)->Result<(),ModuleError>{if module.name.is_empty()||module.version.is_empty(){return Err(ModuleError::InvalidDescriptor)}if self.modules.contains_key(&module.id){return Err(ModuleError::DuplicateId)}self.modules.insert(module.id,module);Ok(())} pub fn get(&self,id:ModuleId)->Option<&ModuleDescriptor>{self.modules.get(&id)} pub fn iter(&self)->impl Iterator<Item=&ModuleDescriptor>{self.modules.values()} }
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum ModuleError{InvalidDescriptor,DuplicateId}
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum LogLevel{Trace,Debug,Info,Warn,Error}
#[derive(Clone,Debug,PartialEq,Eq)] pub struct Diagnostic { pub level:LogLevel,pub code:u32,pub message:String }
#[derive(Default)] pub struct Diagnostics { entries:Vec<Diagnostic> }
impl Diagnostics { pub fn record(&mut self,entry:Diagnostic){self.entries.push(entry)} pub fn entries(&self)->&[Diagnostic]{&self.entries} pub fn drain(&mut self)->Vec<Diagnostic>{std::mem::take(&mut self.entries)} }
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)] pub struct TraceSpanId(pub u64);
#[derive(Default)] pub struct Tracer { next_id:u64,active:Vec<TraceSpanId> }
impl Tracer { pub fn begin(&mut self)->Result<TraceSpanId,TraceError>{let id=TraceSpanId(self.next_id);self.next_id=self.next_id.checked_add(1).ok_or(TraceError::IdOverflow)?;self.active.push(id);Ok(id)} pub fn end(&mut self,id:TraceSpanId)->Result<(),TraceError>{match self.active.pop(){Some(last) if last==id=>Ok(()),Some(last)=>{self.active.push(last);Err(TraceError::OutOfOrder)},None=>Err(TraceError::NoActiveSpan)}} pub fn active_depth(&self)->usize{self.active.len()} }
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum TraceError{IdOverflow,OutOfOrder,NoActiveSpan}
pub trait EnginePlugin: fmt::Debug { fn descriptor(&self)->ModuleDescriptor; fn start(&mut self)->Result<(),PluginError>; fn stop(&mut self)->Result<(),PluginError>; }
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum PluginError{StartFailed,StopFailed,InvalidLifecycle}
#[cfg(test)] mod tests { use super::*; #[test] fn time_is_deterministic(){let mut t=TimeState::default();assert_eq!(t.advance(0.5).unwrap().frame,0);assert_eq!(t.advance(0.25).unwrap().frame,1);assert_eq!(t.frame(),2);} #[test] fn time_rejects_invalid_delta(){let mut t=TimeState::default();assert_eq!(t.advance(-1.0),Err(TimeError::NegativeDelta));assert_eq!(t.advance(f32::NAN),Err(TimeError::NonFiniteDelta));} #[test] fn events_preserve_order(){let mut b=EventBus::default();b.publish(Event{topic:"a".into(),payload:vec![1]}).unwrap();b.publish(Event{topic:"b".into(),payload:vec![2]}).unwrap();assert_eq!(b.drain()[0].topic,"a");} #[test] fn modules_are_unique_and_ordered(){let mut r=ModuleRegistry::default();r.register(ModuleDescriptor{id:ModuleId(2),name:"b".into(),version:"1".into()}).unwrap();r.register(ModuleDescriptor{id:ModuleId(1),name:"a".into(),version:"1".into()}).unwrap();assert_eq!(r.iter().next().unwrap().name,"a");assert_eq!(r.register(ModuleDescriptor{id:ModuleId(1),name:"x".into(),version:"1".into()}),Err(ModuleError::DuplicateId));} #[test] fn diagnostics_are_recorded(){let mut d=Diagnostics::default();d.record(Diagnostic{level:LogLevel::Warn,code:7,message:"test".into()});assert_eq!(d.entries().len(),1);} #[test] fn trace_spans_are_nested(){let mut t=Tracer::default();let a=t.begin().unwrap();let b=t.begin().unwrap();assert_eq!(t.active_depth(),2);assert_eq!(t.end(b),Ok(()));assert_eq!(t.end(a),Ok(()));} #[test] fn trace_rejects_wrong_order(){let mut t=Tracer::default();let a=t.begin().unwrap();let b=t.begin().unwrap();assert_eq!(t.end(a),Err(TraceError::OutOfOrder));assert_eq!(t.end(b),Ok(()));assert_eq!(t.end(a),Ok(()));} }

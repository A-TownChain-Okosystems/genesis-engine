//! Reusable deterministic foundations for genre-scale gameplay systems.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterState { pub grounded: bool, pub stamina: u16, pub max_stamina: u16 }
impl Default for CharacterState { fn default() -> Self { Self { grounded: true, stamina: 100, max_stamina: 100 } } }
impl CharacterState { pub fn jump(&mut self) -> bool { if self.grounded && self.stamina >= 10 { self.grounded=false; self.stamina-=10; true } else { false } } pub fn land(&mut self) { self.grounded=true; } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EconomyBalance { pub credits: i64 }
impl EconomyBalance { pub fn deposit(&mut self, amount:i64)->bool { if amount<0{return false}; self.credits=self.credits.saturating_add(amount); true } pub fn withdraw(&mut self, amount:i64)->bool { if amount<0 || self.credits<amount{return false}; self.credits-=amount; true } }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuildCell { pub x:i32, pub y:i32, pub z:i32, pub occupied:bool }
#[derive(Default)] pub struct BuildGrid { pub cells: Vec<BuildCell> }
impl BuildGrid { pub fn place(&mut self,x:i32,y:i32,z:i32)->bool { if self.cells.iter().any(|c|c.x==x&&c.y==y&&c.z==z&&c.occupied){return false}; self.cells.push(BuildCell{x,y,z,occupied:true}); true } pub fn remove(&mut self,x:i32,y:i32,z:i32)->bool { if let Some(c)=self.cells.iter_mut().find(|c|c.x==x&&c.y==y&&c.z==z&&c.occupied){c.occupied=false;true}else{false} } }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkTick { pub sequence:u64, pub server_tick:u64 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputFrame { pub tick:u64, pub buttons:u32, pub axis:[f32;2] }
impl InputFrame { pub fn normalized(self)->Self { Self{tick:self.tick,buttons:self.buttons,axis:[self.axis[0].clamp(-1.0,1.0),self.axis[1].clamp(-1.0,1.0)]} } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaceState { pub lap:u32, pub checkpoint:u32, pub progress:f32 }
impl RaceState { pub fn advance(&mut self, checkpoint:u32, progress:f32) { if checkpoint>=self.checkpoint { self.checkpoint=checkpoint; self.progress=progress.clamp(0.0,1.0); } } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlightState { pub altitude:f32, pub speed:f32, pub pitch:f32, pub yaw:f32, pub roll:f32 }
impl FlightState { pub fn integrate(&mut self, thrust:f32, lift:f32, dt:f32) { let dt=dt.max(0.0); self.speed=(self.speed+thrust*dt).max(0.0); self.altitude+=(lift*self.speed)*dt; } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceState { pub position:[f64;3], pub velocity:[f64;3] }
impl SpaceState { pub fn integrate(&mut self, acceleration:[f64;3], dt:f64) { let dt=dt.max(0.0); for i in 0..3 { self.velocity[i]+=acceleration[i]*dt; self.position[i]+=self.velocity[i]*dt; } } }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NarrativeEvent { Start(u32), Choice(u32), Complete(u32) }
#[derive(Default)] pub struct NarrativeState { pub events: Vec<NarrativeEvent> }
impl NarrativeState { pub fn push(&mut self,e:NarrativeEvent){self.events.push(e);} pub fn completed(&self,id:u32)->bool{self.events.contains(&NarrativeEvent::Complete(id))} }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrategyOrder { pub entity:u64, pub target:u64, pub priority:u8 }
#[derive(Default)] pub struct StrategyQueue { pub orders: Vec<StrategyOrder> }
impl StrategyQueue { pub fn push(&mut self,o:StrategyOrder){self.orders.push(o);self.orders.sort_by_key(|x| (x.priority,x.entity,x.target));} pub fn pop(&mut self)->Option<StrategyOrder>{if self.orders.is_empty(){None}else{Some(self.orders.remove(0))}} }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModDescriptor { pub id:String, pub version:String, pub dependencies:Vec<String> }
#[derive(Default)] pub struct ModRegistry { pub mods:Vec<ModDescriptor> }
impl ModRegistry { pub fn register(&mut self,m:ModDescriptor)->bool { if m.id.is_empty()||self.mods.iter().any(|x|x.id==m.id){return false}; self.mods.push(m); self.mods.sort_by(|a,b|a.id.cmp(&b.id)); true } pub fn contains(&self,id:&str)->bool{self.mods.iter().any(|m|m.id==id)} }

#[cfg(test)] mod tests { use super::*;
#[test] fn character_jump_is_gated(){let mut c=CharacterState::default();assert!(c.jump());assert!(!c.jump());c.land();assert_eq!(c.stamina,90);}
#[test] fn economy_never_overdraws(){let mut e=EconomyBalance{credits:10};assert!(!e.withdraw(11));assert_eq!(e.credits,10);}
#[test] fn building_is_deterministic(){let mut g=BuildGrid::default();assert!(g.place(1,2,3));assert!(!g.place(1,2,3));assert!(g.remove(1,2,3));}
#[test] fn network_input_is_clamped(){let f=InputFrame{tick:1,buttons:0,axis:[3.0,-3.0]}.normalized();assert_eq!(f.axis,[1.0,-1.0]);}
#[test] fn strategy_order_is_stable(){let mut q=StrategyQueue::default();q.push(StrategyOrder{entity:2,target:3,priority:1});q.push(StrategyOrder{entity:1,target:4,priority:1});assert_eq!(q.pop().unwrap().entity,1);}
#[test] fn mods_are_unique(){let mut r=ModRegistry::default();let m=ModDescriptor{id:"core".into(),version:"1".into(),dependencies:vec![]};assert!(r.register(m.clone()));assert!(!r.register(m));}
}
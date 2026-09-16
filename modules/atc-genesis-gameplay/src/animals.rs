//! Deterministic animal simulation foundation for farming, wildlife and ecosystem gameplay.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalKind { Herbivore, Carnivore, Omnivore, Bird, Aquatic, Livestock, Insect, Fantasy }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalState { Idle, Roaming, Grazing, Hunting, Fleeing, Sleeping, Following, Dead }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimalDefinition { pub id: &'static str, pub kind: AnimalKind, pub max_health: u16, pub move_speed_mm_s: u32, pub hunger_per_tick: u16, pub reproduction_cooldown: u32 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimalEnvironment { pub food: u16, pub water: u16, pub threat: u16, pub temperature: i16 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimalInstance { pub species: &'static str, pub state: AnimalState, pub health: u16, pub hunger: u16, pub age_ticks: u32, pub reproduction_timer: u32 }
impl AnimalInstance {
 pub fn new(species:&'static str,max_health:u16)->Self{Self{species,state:AnimalState::Idle,health:max_health,hunger:0,age_ticks:0,reproduction_timer:0}}
 pub fn tick(&mut self,def:AnimalDefinition,env:AnimalEnvironment){if self.state==AnimalState::Dead||self.species!=def.id{return;}self.age_ticks=self.age_ticks.saturating_add(1);self.hunger=self.hunger.saturating_add(def.hunger_per_tick);self.reproduction_timer=self.reproduction_timer.saturating_sub(1);self.state=if env.threat>700{AnimalState::Fleeing}else if self.hunger>700&&env.food>0{AnimalState::Grazing}else{AnimalState::Roaming};if self.hunger>900{self.health=self.health.saturating_sub(10);}if self.health==0{self.state=AnimalState::Dead;}}
 pub fn feed(&mut self,amount:u16){if self.state!=AnimalState::Dead{self.hunger=self.hunger.saturating_sub(amount);}}
 pub fn drink(&mut self,amount:u16){if self.state!=AnimalState::Dead{self.hunger=self.hunger.saturating_sub(amount/2);}}
 pub fn can_reproduce(&self,def:AnimalDefinition,partner:&Self)->bool{self.state!=AnimalState::Dead&&partner.state!=AnimalState::Dead&&self.species==partner.species&&self.reproduction_timer==0&&partner.reproduction_timer==0&&self.health>def.max_health/2&&partner.health>def.max_health/2}
 pub fn reproduce(&mut self,partner:&mut Self,def:AnimalDefinition)->Option<Self>{if !self.can_reproduce(def,partner){return None;}self.reproduction_timer=def.reproduction_cooldown;partner.reproduction_timer=def.reproduction_cooldown;Some(Self::new(def.id,def.max_health))}
}
#[derive(Default,Debug,PartialEq,Eq)]
pub struct Herd{pub animals:Vec<AnimalInstance>,pub capacity:usize}
impl Herd{pub fn new(capacity:usize)->Self{Self{animals:Vec::new(),capacity}}pub fn add(&mut self,animal:AnimalInstance)->bool{if self.animals.len()>=self.capacity{return false;}self.animals.push(animal);true}}
pub const DEER:AnimalDefinition=AnimalDefinition{id:"deer",kind:AnimalKind::Herbivore,max_health:100,move_speed_mm_s:6500,hunger_per_tick:5,reproduction_cooldown:120};
pub const WOLF:AnimalDefinition=AnimalDefinition{id:"wolf",kind:AnimalKind::Carnivore,max_health:120,move_speed_mm_s:8000,hunger_per_tick:6,reproduction_cooldown:160};
pub const COW:AnimalDefinition=AnimalDefinition{id:"cow",kind:AnimalKind::Livestock,max_health:180,move_speed_mm_s:3500,hunger_per_tick:4,reproduction_cooldown:240};
#[cfg(test)]mod tests{use super::*;#[test]fn threat_causes_fleeing(){let mut a=AnimalInstance::new("deer",DEER.max_health);a.tick(DEER,AnimalEnvironment{food:100,water:100,threat:800,temperature:20});assert_eq!(a.state,AnimalState::Fleeing)}#[test]fn hunger_causes_grazing(){let mut a=AnimalInstance::new("deer",DEER.max_health);a.hunger=800;a.tick(DEER,AnimalEnvironment{food:100,water:100,threat:0,temperature:20});assert_eq!(a.state,AnimalState::Grazing)}#[test]fn reproduction_is_deterministic(){let mut a=AnimalInstance::new("cow",COW.max_health);let mut b=AnimalInstance::new("cow",COW.max_health);assert!(a.reproduce(&mut b,COW).is_some());assert_eq!(a.reproduction_timer,COW.reproduction_cooldown)}#[test]fn herd_capacity_is_enforced(){let mut h=Herd::new(1);assert!(h.add(AnimalInstance::new("deer",DEER.max_health)));assert!(!h.add(AnimalInstance::new("deer",DEER.max_health)))}}

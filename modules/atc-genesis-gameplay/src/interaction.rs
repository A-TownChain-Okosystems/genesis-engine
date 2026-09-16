//! Reusable interaction and gameplay-ability primitives.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionKind { Use, Talk, PickUp, Open, Activate, Inspect }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interaction { pub source: u64, pub target: u64, pub kind: InteractionKind }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbilityState { Ready, Active, Cooldown }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ability { pub id: u32, pub duration: f32, pub cooldown: f32, pub state: AbilityState, pub remaining: f32 }
impl Ability {
    pub fn new(id: u32, duration: f32, cooldown: f32) -> Self { Self { id, duration: duration.max(0.0), cooldown: cooldown.max(0.0), state: AbilityState::Ready, remaining: 0.0 } }
    pub fn activate(&mut self) -> bool { if self.state != AbilityState::Ready { return false; } self.state=if self.duration>0.0 {AbilityState::Active} else {AbilityState::Cooldown}; self.remaining=if self.duration>0.0 {self.duration} else {self.cooldown}; true }
    pub fn tick(&mut self, dt: f32) { let mut t=dt.max(0.0); while t>0.0 { match self.state { AbilityState::Ready => break, AbilityState::Active => { let step=t.min(self.remaining); self.remaining-=step; t-=step; if self.remaining<=0.0 { self.state=AbilityState::Cooldown; self.remaining=self.cooldown; } }, AbilityState::Cooldown => { let step=t.min(self.remaining); self.remaining-=step; t-=step; if self.remaining<=0.0 { self.state=AbilityState::Ready; } } } } }
}

#[cfg(test)]
mod tests { use super::*;
#[test] fn ability_lifecycle_is_deterministic(){let mut a=Ability::new(7,1.0,2.0);assert!(a.activate());assert!(!a.activate());a.tick(1.0);assert_eq!(a.state,AbilityState::Cooldown);a.tick(2.0);assert_eq!(a.state,AbilityState::Ready);}
#[test] fn interaction_is_explicit(){let i=Interaction{source:1,target:2,kind:InteractionKind::Talk};assert_eq!(i.kind,InteractionKind::Talk);}
}

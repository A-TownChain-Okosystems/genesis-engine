//! Deterministic, engine-agnostic combat primitives.
//!
//! This module is a reusable foundation for action, RPG, shooter, fighting,
//! tactics, survival and simulation genres. Rendering, animation and network
//! transport remain outside the combat authority.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageType { Physical, Fire, Cold, Electric, Poison, True }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage { pub amount: f32, pub kind: DamageType }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health { pub current: f32, pub maximum: f32 }
impl Health {
    pub fn new(maximum: f32) -> Self { let m = maximum.max(0.0); Self { current: m, maximum: m } }
    pub fn alive(&self) -> bool { self.current > 0.0 }
    pub fn apply(&mut self, damage: Damage, resistance: f32) -> f32 {
        let raw = damage.amount.max(0.0);
        let mitigated = match damage.kind { DamageType::True => raw, _ => raw * (1.0 - resistance.clamp(0.0, 1.0)) };
        let dealt = mitigated.min(self.current.max(0.0));
        self.current = (self.current - dealt).max(0.0);
        dealt
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attack { pub damage: Damage, pub range: f32, pub cooldown: f32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Combatant { pub health: Health, pub resistance: f32, pub cooldown_remaining: f32 }
impl Combatant {
    pub fn tick(&mut self, dt: f32) { self.cooldown_remaining = (self.cooldown_remaining - dt.max(0.0)).max(0.0); }
    pub fn can_attack(&self) -> bool { self.health.alive() && self.cooldown_remaining <= 0.0 }
    pub fn attack(&mut self, target: &mut Combatant, attack: Attack) -> Option<f32> {
        if !self.can_attack() || !target.health.alive() || attack.range < 0.0 { return None; }
        self.cooldown_remaining = attack.cooldown.max(0.0);
        Some(target.health.apply(attack.damage, target.resistance))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamRelation { Ally, Neutral, Enemy }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CombatEvent { pub source: u64, pub target: u64, pub damage: f32, pub target_alive: bool }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn resistance_is_applied_deterministically() { let mut h=Health::new(100.0); assert_eq!(h.apply(Damage{amount:50.0,kind:DamageType::Physical},0.2),40.0); assert_eq!(h.current,60.0); }
    #[test] fn true_damage_ignores_resistance() { let mut h=Health::new(25.0); assert_eq!(h.apply(Damage{amount:30.0,kind:DamageType::True},1.0),25.0); assert!(!h.alive()); }
    #[test] fn cooldown_blocks_repeated_attacks() { let mut a=Combatant{health:Health::new(10.0),resistance:0.0,cooldown_remaining:0.0}; let mut b=a; let x=Attack{damage:Damage{amount:3.0,kind:DamageType::Physical},range:1.0,cooldown:1.0}; assert_eq!(a.attack(&mut b,x),Some(3.0)); assert_eq!(a.attack(&mut b,x),None); a.tick(1.0); assert_eq!(a.attack(&mut b,x),Some(3.0)); }
}

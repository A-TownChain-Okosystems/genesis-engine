//! Deterministic production stations and tool durability.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionStationKind { Workbench, Forge, Kitchen, Loom, Laboratory }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionStation {
    pub id: &'static str,
    pub kind: ProductionStationKind,
    pub capacity: u16,
    pub enabled: bool,
}

impl ProductionStation {
    pub fn accepts(&self, station_id: &str) -> bool { self.enabled && self.id == station_id && self.capacity > 0 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolState { pub tool_id: &'static str, pub durability: u32, pub max_durability: u32 }

impl ToolState {
    pub fn new(tool_id: &'static str, max_durability: u32) -> Option<Self> {
        if tool_id.is_empty() || max_durability == 0 { return None; }
        Some(Self { tool_id, durability: max_durability, max_durability })
    }
    pub fn use_once(&mut self) -> bool {
        if self.durability == 0 { return false; }
        self.durability -= 1;
        true
    }
    pub fn broken(&self) -> bool { self.durability == 0 }
    pub fn condition_permille(&self) -> u16 {
        if self.max_durability == 0 { return 0; }
        ((self.durability as u64 * 1000 / self.max_durability as u64).min(1000)) as u16
    }
}

pub const WORKBENCH: ProductionStation = ProductionStation { id: "workbench", kind: ProductionStationKind::Workbench, capacity: 1, enabled: true };
pub const FORGE: ProductionStation = ProductionStation { id: "forge", kind: ProductionStationKind::Forge, capacity: 1, enabled: true };
pub const KITCHEN: ProductionStation = ProductionStation { id: "kitchen", kind: ProductionStationKind::Kitchen, capacity: 1, enabled: true };

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn station_gate_is_deterministic() { assert!(WORKBENCH.accepts("workbench")); assert!(!WORKBENCH.accepts("forge")); }
    #[test] fn tool_wear_is_bounded() { let mut t=ToolState::new("hammer",2).unwrap(); assert!(t.use_once()); assert_eq!(t.condition_permille(),500); assert!(t.use_once()); assert!(!t.use_once()); assert!(t.broken()); }
    #[test] fn invalid_tool_is_rejected() { assert!(ToolState::new("",1).is_none()); assert!(ToolState::new("hammer",0).is_none()); }
}

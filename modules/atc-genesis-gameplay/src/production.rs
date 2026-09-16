//! Deterministic production stations, bounded storage, and tool durability.

use crate::items::Inventory;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionStationKind { Workbench, Forge, Kitchen, Loom, Laboratory }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionStation {
    pub id: &'static str,
    pub kind: ProductionStationKind,
    pub capacity: u16,
    pub enabled: bool,
    pub input: Inventory,
    pub output: Inventory,
}

impl ProductionStation {
    pub fn new(id: &'static str, kind: ProductionStationKind, capacity: u16) -> Option<Self> {
        if id.is_empty() || capacity == 0 { return None; }
        Some(Self { id, kind, capacity, enabled: true, input: Inventory::default(), output: Inventory::default() })
    }
    pub fn accepts(&self, station_id: &str) -> bool { self.enabled && self.id == station_id && self.capacity > 0 }
    pub fn store_input(&mut self, item_id: &str, quantity: u32, stack_limit: u32) -> u32 {
        if !self.enabled || self.capacity == 0 || item_id.is_empty() || quantity == 0 || stack_limit == 0 { return quantity; }
        self.input.add(item_id, quantity, stack_limit)
    }
    pub fn take_input(&mut self, item_id: &str, quantity: u32) -> bool { self.input.remove(item_id, quantity) }
    pub fn store_output(&mut self, item_id: &str, quantity: u32, stack_limit: u32) -> u32 {
        if !self.enabled || self.capacity == 0 || item_id.is_empty() || quantity == 0 || stack_limit == 0 { return quantity; }
        self.output.add(item_id, quantity, stack_limit)
    }
    pub fn take_output(&mut self, item_id: &str, quantity: u32) -> bool { self.output.remove(item_id, quantity) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolState { pub tool_id: &'static str, pub durability: u32, pub max_durability: u32 }

impl ToolState {
    pub fn new(tool_id: &'static str, max_durability: u32) -> Option<Self> {
        if tool_id.is_empty() || max_durability == 0 { return None; }
        Some(Self { tool_id, durability: max_durability, max_durability })
    }
    pub fn use_once(&mut self) -> bool { if self.durability == 0 { return false; } self.durability -= 1; true }
    pub fn broken(&self) -> bool { self.durability == 0 }
    pub fn condition_permille(&self) -> u16 {
        if self.max_durability == 0 { return 0; }
        ((self.durability as u64 * 1000 / self.max_durability as u64).min(1000)) as u16
    }
}

pub const WORKBENCH: ProductionStation = ProductionStation { id: "workbench", kind: ProductionStationKind::Workbench, capacity: 1, enabled: true, input: Inventory { stacks: Vec::new() }, output: Inventory { stacks: Vec::new() } };
pub const FORGE: ProductionStation = ProductionStation { id: "forge", kind: ProductionStationKind::Forge, capacity: 1, enabled: true, input: Inventory { stacks: Vec::new() }, output: Inventory { stacks: Vec::new() } };
pub const KITCHEN: ProductionStation = ProductionStation { id: "kitchen", kind: ProductionStationKind::Kitchen, capacity: 1, enabled: true, input: Inventory { stacks: Vec::new() }, output: Inventory { stacks: Vec::new() } };

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn station_gate_is_deterministic() { let s=ProductionStation::new("workbench", WORKBENCH.kind, 1).unwrap(); assert!(s.accepts("workbench")); assert!(!s.accepts("forge")); }
    #[test] fn station_storage_is_atomic_on_invalid_input() { let mut s=ProductionStation::new("workbench", WORKBENCH.kind, 1).unwrap(); assert_eq!(s.store_input("wood",4,10),0); assert_eq!(s.input.count("wood"),4); assert!(s.take_input("wood",2)); assert_eq!(s.input.count("wood"),2); assert_eq!(s.store_output("plank",3,10),0); assert!(s.take_output("plank",3)); }
    #[test] fn disabled_station_rejects_storage() { let mut s=ProductionStation::new("workbench", WORKBENCH.kind, 1).unwrap(); s.enabled=false; assert_eq!(s.store_input("wood",1,10),1); }
    #[test] fn tool_wear_is_bounded() { let mut t=ToolState::new("hammer",2).unwrap(); assert!(t.use_once()); assert_eq!(t.condition_permille(),500); assert!(t.use_once()); assert!(!t.use_once()); assert!(t.broken()); }
    #[test] fn invalid_tool_is_rejected() { assert!(ToolState::new("",1).is_none()); assert!(ToolState::new("hammer",0).is_none()); }
}

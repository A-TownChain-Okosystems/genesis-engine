//! Deterministic item, weapon and ammunition foundations.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind { Consumable, Material, Quest, Currency, Weapon, Armor, Tool, Misc }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponKind { Melee, Pistol, Rifle, Shotgun, Launcher, Energy, Magic, Tool }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageType { Physical, Fire, Cold, Electric, Poison, Arcane, True }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemDefinition { pub id: String, pub kind: ItemKind, pub stack_limit: u32, pub weight_mg: u32 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeaponDefinition {
    pub id: String,
    pub kind: WeaponKind,
    pub damage_type: DamageType,
    pub damage: u32,
    pub range_mm: u32,
    pub magazine_size: u32,
    pub ammo_id: Option<String>,
    pub durability: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemStack { pub item_id: String, pub quantity: u32 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeaponInstance { pub weapon_id: String, pub ammo: u32, pub durability: u32 }

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Inventory { pub stacks: Vec<ItemStack> }
impl Inventory {
    pub fn add(&mut self, item_id: &str, quantity: u32, stack_limit: u32) -> u32 {
        if item_id.is_empty() || quantity == 0 || stack_limit == 0 { return quantity; }
        let mut remaining = quantity;
        for stack in &mut self.stacks {
            if stack.item_id == item_id && stack.quantity < stack_limit {
                let add = remaining.min(stack_limit - stack.quantity); stack.quantity += add; remaining -= add;
                if remaining == 0 { return 0; }
            }
        }
        while remaining > 0 { let add = remaining.min(stack_limit); self.stacks.push(ItemStack { item_id: item_id.into(), quantity: add }); remaining -= add; }
        0
    }
    pub fn remove(&mut self, item_id: &str, quantity: u32) -> bool {
        if quantity == 0 { return true; }
        let total: u32 = self.stacks.iter().filter(|s| s.item_id == item_id).map(|s| s.quantity).sum();
        if total < quantity { return false; }
        let mut remaining = quantity;
        for stack in &mut self.stacks { if stack.item_id == item_id && remaining > 0 { let take = remaining.min(stack.quantity); stack.quantity -= take; remaining -= take; } }
        self.stacks.retain(|s| s.quantity > 0); true
    }
    pub fn count(&self, item_id: &str) -> u32 { self.stacks.iter().filter(|s| s.item_id == item_id).map(|s| s.quantity).sum() }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct WeaponLoadout { pub slots: Vec<Option<WeaponInstance>> }
impl WeaponLoadout {
    pub fn with_slots(count: usize) -> Self { Self { slots: vec![None; count] } }
    pub fn equip(&mut self, slot: usize, weapon: WeaponInstance) -> bool { if let Some(target) = self.slots.get_mut(slot) { *target = Some(weapon); true } else { false } }
    pub fn unequip(&mut self, slot: usize) -> Option<WeaponInstance> { self.slots.get_mut(slot).and_then(Option::take) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeaponFireResult { pub fired: bool, pub consumed_ammo: u32, pub remaining_ammo: u32 }
impl WeaponInstance {
    pub fn fire(&mut self) -> WeaponFireResult {
        if self.ammo == 0 || self.durability == 0 { return WeaponFireResult { fired: false, consumed_ammo: 0, remaining_ammo: self.ammo }; }
        self.ammo -= 1; self.durability -= 1;
        WeaponFireResult { fired: true, consumed_ammo: 1, remaining_ammo: self.ammo }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn inventory_stacks_deterministically() { let mut i = Inventory::default(); assert_eq!(i.add("potion", 7, 5), 0); assert_eq!(i.count("potion"), 7); assert_eq!(i.stacks.len(), 2); }
    #[test] fn inventory_does_not_over_remove() { let mut i = Inventory::default(); i.add("ore", 3, 10); assert!(!i.remove("ore", 4)); assert_eq!(i.count("ore"), 3); }
    #[test] fn loadout_respects_slot_bounds() { let mut l = WeaponLoadout::with_slots(2); let w = WeaponInstance { weapon_id: "rifle".into(), ammo: 5, durability: 10 }; assert!(l.equip(1, w.clone())); assert!(!l.equip(2, w)); assert_eq!(l.unequip(1).unwrap().weapon_id, "rifle"); }
    #[test] fn weapon_fire_is_deterministic() { let mut w = WeaponInstance { weapon_id: "pistol".into(), ammo: 1, durability: 1 }; assert_eq!(w.fire(), WeaponFireResult { fired: true, consumed_ammo: 1, remaining_ammo: 0 }); assert!(!w.fire().fired); }
}

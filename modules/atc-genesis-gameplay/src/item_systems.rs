//! Data-driven extensions for items and weapons.

use crate::{Inventory, WeaponInstance};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rarity { Common, Uncommon, Rare, Epic, Legendary }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorSlot { Head, Chest, Hands, Legs, Feet, Shield }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArmorStats { pub physical: u32, pub fire: u32, pub cold: u32, pub electric: u32, pub poison: u32 }

impl ArmorStats {
    pub fn mitigate(self, damage: u32, damage_type: crate::ItemDamageType) -> u32 {
        let resistance = match damage_type { crate::ItemDamageType::Physical => self.physical, crate::ItemDamageType::Fire => self.fire, crate::ItemDamageType::Cold => self.cold, crate::ItemDamageType::Electric => self.electric, crate::ItemDamageType::Poison => self.poison, _ => 0 };
        damage.saturating_mul(100u32.saturating_sub(resistance.min(100))) / 100
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeaponAttachment { pub damage_bonus: i32, pub range_bonus_mm: i32, pub magazine_bonus: i32 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ammunition { pub damage_multiplier_percent: u32, pub damage_type: crate::ItemDamageType }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LootEntry { pub item_id: &'static str, pub weight: u32, pub min: u32, pub max: u32 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LootTable { pub entries: Vec<LootEntry> }
impl LootTable {
    pub fn roll(&self, seed: u64) -> Option<LootEntry> {
        let total: u64 = self.entries.iter().map(|e| e.weight as u64).sum();
        if total == 0 { return None; }
        let mut x = seed ^ 0x9E3779B97F4A7C15;
        x ^= x >> 30; x = x.wrapping_mul(0xBF58476D1CE4E5B9); x ^= x >> 27; x = x.wrapping_mul(0x94D049BB133111EB); x ^= x >> 31;
        let mut cursor = x % total;
        for entry in &self.entries { if cursor < entry.weight as u64 { return Some(*entry); } cursor -= entry.weight as u64; }
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecipeIngredient { pub item_id: String, pub quantity: u32 }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recipe { pub id: String, pub ingredients: Vec<RecipeIngredient>, pub output_item_id: String, pub output_quantity: u32 }
impl Recipe {
    pub fn craft(&self, inventory: &mut Inventory) -> bool {
        if self.output_quantity == 0 || self.ingredients.iter().any(|i| i.quantity == 0 || inventory.count(&i.item_id) < i.quantity) { return false; }
        for ingredient in &self.ingredients { inventory.remove(&ingredient.item_id, ingredient.quantity); }
        inventory.add(&self.output_item_id, self.output_quantity, u32::MAX); true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeaponAttachmentState { pub weapon: WeaponInstance, pub attachments: Vec<WeaponAttachment> }
impl WeaponAttachmentState {
    pub fn damage_bonus(&self) -> i32 { self.attachments.iter().map(|a| a.damage_bonus).sum() }
    pub fn range_bonus_mm(&self) -> i32 { self.attachments.iter().map(|a| a.range_bonus_mm).sum() }
    pub fn magazine_bonus(&self) -> i32 { self.attachments.iter().map(|a| a.magazine_bonus).sum() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn armor_mitigation_is_bounded() { let a=ArmorStats{physical:150,fire:50,cold:0,electric:0,poison:0}; assert_eq!(a.mitigate(100,crate::ItemDamageType::Physical),0); assert_eq!(a.mitigate(100,crate::ItemDamageType::Fire),50); }
    #[test] fn loot_roll_is_deterministic() { let t=LootTable{entries:vec![LootEntry{item_id:"a",weight:1,min:1,max:1},LootEntry{item_id:"b",weight:3,min:1,max:2}]}; assert_eq!(t.roll(42),t.roll(42)); }
    #[test] fn crafting_is_atomic_on_missing_ingredients() { let r=Recipe{id:"r".into(),ingredients:vec![RecipeIngredient{item_id:"ore".into(),quantity:2}],output_item_id:"bar".into(),output_quantity:1}; let mut i=Inventory::default(); i.add("ore",1,10); assert!(!r.craft(&mut i)); assert_eq!(i.count("ore"),1); }
    #[test] fn attachments_sum_modifiers() { let s=WeaponAttachmentState{weapon:WeaponInstance{weapon_id:"x".into(),ammo:1,durability:1},attachments:vec![WeaponAttachment{damage_bonus:2,range_bonus_mm:10,magazine_bonus:1},WeaponAttachment{damage_bonus:3,range_bonus_mm:-2,magazine_bonus:0}]}; assert_eq!(s.damage_bonus(),5); assert_eq!(s.range_bonus_mm(),8); }
}

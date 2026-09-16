//! Reusable presets for common game-design structures.
//! Presets are data-oriented defaults, not genre-specific runtime logic.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryPreset { pub name: &'static str, pub slots: u16, pub hotbar_slots: u16, pub stack_limit: u16 }
impl InventoryPreset {
    pub const RPG: Self = Self { name: "rpg", slots: 40, hotbar_slots: 8, stack_limit: 99 };
    pub const SURVIVAL: Self = Self { name: "survival", slots: 24, hotbar_slots: 8, stack_limit: 64 };
    pub const MMO: Self = Self { name: "mmo", slots: 80, hotbar_slots: 12, stack_limit: 999 };
    pub const TACTICAL: Self = Self { name: "tactical", slots: 16, hotbar_slots: 4, stack_limit: 10 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharacterSlotPreset { pub name: &'static str, pub character_slots: u8, pub active_slots: u8, pub party_slots: u8 }
impl CharacterSlotPreset {
    pub const SINGLE: Self = Self { name: "single", character_slots: 1, active_slots: 1, party_slots: 1 };
    pub const RPG_PARTY: Self = Self { name: "rpg_party", character_slots: 12, active_slots: 1, party_slots: 4 };
    pub const MMO_ROSTER: Self = Self { name: "mmo_roster", character_slots: 8, active_slots: 1, party_slots: 5 };
    pub const SQUAD: Self = Self { name: "squad", character_slots: 16, active_slots: 4, party_slots: 4 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkillTreePreset { pub name: &'static str, pub max_nodes: u16, pub starting_points: u16, pub branches: u8, pub max_rank: u8 }
impl SkillTreePreset {
    pub const SIMPLE: Self = Self { name: "simple", max_nodes: 30, starting_points: 0, branches: 3, max_rank: 5 };
    pub const RPG: Self = Self { name: "rpg", max_nodes: 120, starting_points: 1, branches: 6, max_rank: 10 };
    pub const CLASSIC: Self = Self { name: "classic", max_nodes: 60, starting_points: 0, branches: 3, max_rank: 5 };
    pub const MMO: Self = Self { name: "mmo", max_nodes: 240, starting_points: 1, branches: 12, max_rank: 20 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AbilityBarPreset { pub name: &'static str, pub slots: u8, pub ultimate_slots: u8, pub passive_slots: u8 }
impl AbilityBarPreset {
    pub const ACTION: Self = Self { name: "action", slots: 8, ultimate_slots: 1, passive_slots: 4 };
    pub const RPG: Self = Self { name: "rpg", slots: 12, ultimate_slots: 2, passive_slots: 6 };
    pub const MMO: Self = Self { name: "mmo", slots: 24, ultimate_slots: 2, passive_slots: 10 };
    pub const HERO_SHOOTER: Self = Self { name: "hero_shooter", slots: 4, ultimate_slots: 1, passive_slots: 2 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquipmentPreset { pub name: &'static str, pub weapon_slots: u8, pub armor_slots: u8, pub accessory_slots: u8 }
impl EquipmentPreset {
    pub const RPG: Self = Self { name: "rpg", weapon_slots: 2, armor_slots: 6, accessory_slots: 4 };
    pub const SURVIVAL: Self = Self { name: "survival", weapon_slots: 2, armor_slots: 4, accessory_slots: 2 };
    pub const TACTICAL: Self = Self { name: "tactical", weapon_slots: 2, armor_slots: 5, accessory_slots: 1 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadoutPreset { pub name: &'static str, pub inventory: InventoryPreset, pub abilities: AbilityBarPreset, pub equipment: EquipmentPreset }
impl LoadoutPreset {
    pub const RPG: Self = Self { name: "rpg", inventory: InventoryPreset::RPG, abilities: AbilityBarPreset::RPG, equipment: EquipmentPreset::RPG };
    pub const SURVIVAL: Self = Self { name: "survival", inventory: InventoryPreset::SURVIVAL, abilities: AbilityBarPreset::ACTION, equipment: EquipmentPreset::SURVIVAL };
    pub const TACTICAL: Self = Self { name: "tactical", inventory: InventoryPreset::TACTICAL, abilities: AbilityBarPreset::HERO_SHOOTER, equipment: EquipmentPreset::TACTICAL };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn rpg_loadout_is_consistent() { assert_eq!(LoadoutPreset::RPG.inventory.slots, 40); assert_eq!(LoadoutPreset::RPG.abilities.slots, 12); }
    #[test] fn presets_have_positive_capacity() { assert!(InventoryPreset::MMO.slots > 0); assert!(SkillTreePreset::MMO.max_nodes > 0); }
}

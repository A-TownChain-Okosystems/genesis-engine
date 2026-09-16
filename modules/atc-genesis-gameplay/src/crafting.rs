//! Deterministic, data-driven crafting foundation with station and technology gates.

use crate::Inventory;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CraftingIngredient { pub item_id: &'static str, pub quantity: u32 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CraftingRecipe {
    pub id: &'static str,
    pub ingredients: &'static [CraftingIngredient],
    pub output_item_id: &'static str,
    pub output_quantity: u32,
    pub station: Option<&'static str>,
    pub required_technology: Option<&'static str>,
    pub duration_ticks: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CraftFailure { InvalidRecipe, MissingIngredient, WrongStation, MissingTechnology }

impl CraftingRecipe {
    pub fn validate(&self, inventory: &Inventory, station: Option<&str>, researched: &[&str]) -> Result<(), CraftFailure> {
        if self.id.is_empty() || self.output_item_id.is_empty() || self.output_quantity == 0 || self.ingredients.iter().any(|i| i.item_id.is_empty() || i.quantity == 0) { return Err(CraftFailure::InvalidRecipe); }
        if self.station.is_some() && self.station != station { return Err(CraftFailure::WrongStation); }
        if let Some(tech) = self.required_technology { if !researched.contains(&tech) { return Err(CraftFailure::MissingTechnology); } }
        if self.ingredients.iter().any(|i| inventory.count(i.item_id) < i.quantity) { return Err(CraftFailure::MissingIngredient); }
        Ok(())
    }

    pub fn craft(&self, inventory: &mut Inventory, station: Option<&str>, researched: &[&str]) -> Result<(), CraftFailure> {
        self.validate(inventory, station, researched)?;
        for ingredient in self.ingredients { if !inventory.remove(ingredient.item_id, ingredient.quantity) { return Err(CraftFailure::MissingIngredient); } }
        inventory.add(self.output_item_id, self.output_quantity, u32::MAX);
        Ok(())
    }
}

pub const BASIC_TOOLS: &[CraftingIngredient] = &[
    CraftingIngredient { item_id: "wood", quantity: 2 },
    CraftingIngredient { item_id: "stone", quantity: 2 },
];

pub const BASIC_TOOLS_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "basic_tools", ingredients: BASIC_TOOLS, output_item_id: "stone_tool", output_quantity: 1,
    station: None, required_technology: Some("construction"), duration_ticks: 20,
};

pub const WORKBENCH_RECIPE: CraftingRecipe = CraftingRecipe {
    id: "workbench", ingredients: &[CraftingIngredient { item_id: "wood", quantity: 8 }, CraftingIngredient { item_id: "stone", quantity: 4 }],
    output_item_id: "workbench", output_quantity: 1, station: None, required_technology: Some("construction"), duration_ticks: 50,
};

#[cfg(test)]
mod tests {
    use super::*;
    fn stock() -> Inventory { let mut i=Inventory::default(); i.add("wood",20,100); i.add("stone",20,100); i }
    #[test] fn crafting_consumes_inputs_and_creates_output() { let mut i=stock(); assert!(BASIC_TOOLS_RECIPE.craft(&mut i,None,&["construction"]).is_ok()); assert_eq!(i.count("wood"),18); assert_eq!(i.count("stone"),18); assert_eq!(i.count("stone_tool"),1); }
    #[test] fn technology_is_enforced() { let mut i=stock(); assert_eq!(BASIC_TOOLS_RECIPE.craft(&mut i,None,&[]),Err(CraftFailure::MissingTechnology)); assert_eq!(i.count("wood"),20); }
    #[test] fn station_is_enforced() { let recipe=CraftingRecipe { station:Some("forge"), ..BASIC_TOOLS_RECIPE }; let mut i=stock(); assert_eq!(recipe.craft(&mut i,None,&["construction"]),Err(CraftFailure::WrongStation)); }
    #[test] fn missing_ingredients_do_not_mutate_inventory() { let mut i=Inventory::default(); i.add("wood",1,100); assert_eq!(BASIC_TOOLS_RECIPE.craft(&mut i,None,&["construction"]),Err(CraftFailure::MissingIngredient)); assert_eq!(i.count("wood"),1); }
}

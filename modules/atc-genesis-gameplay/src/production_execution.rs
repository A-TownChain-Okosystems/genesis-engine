//! Atomic deterministic production execution: validate, consume, queue, complete.

use crate::{CraftingJob, CraftingRecipe, Inventory, ProductionStation, ToolState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionFailure { InvalidRecipe, MissingIngredient, WrongStation, MissingTechnology, MissingTool, ToolBroken, OutputBlocked }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionJob {
    pub job: CraftingJob,
    pub output_item_id: String,
    pub output_quantity: u32,
    pub tool_id: Option<String>,
}

pub fn start_production(
    recipe: &CraftingRecipe,
    inventory: &mut Inventory,
    station: &ProductionStation,
    researched: &[&str],
    mut tool: Option<&mut ToolState>,
) -> Result<ProductionJob, ProductionFailure> {
    if recipe.id.is_empty() || recipe.output_item_id.is_empty() || recipe.output_quantity == 0 || recipe.duration_ticks == 0 || recipe.ingredients.iter().any(|i| i.item_id.is_empty() || i.quantity == 0) { return Err(ProductionFailure::InvalidRecipe); }
    if !station.accepts(station.id) || recipe.station.map(|id| id != station.id).unwrap_or(false) { return Err(ProductionFailure::WrongStation); }
    if let Some(tech) = recipe.required_technology { if !researched.contains(&tech) { return Err(ProductionFailure::MissingTechnology); } }
    if recipe.ingredients.iter().any(|i| inventory.count(i.item_id) < i.quantity) { return Err(ProductionFailure::MissingIngredient); }
    if let Some(ref mut t) = tool { if t.broken() { return Err(ProductionFailure::ToolBroken); } }
    let job = CraftingJob::new(recipe).ok_or(ProductionFailure::InvalidRecipe)?;
    for ingredient in recipe.ingredients { if !inventory.remove(ingredient.item_id, ingredient.quantity) { return Err(ProductionFailure::MissingIngredient); } }
    let tool_id = tool.as_ref().map(|t| t.tool_id.to_string());
    if let Some(t) = tool { if !t.use_once() { return Err(ProductionFailure::ToolBroken); } }
    Ok(ProductionJob { job, output_item_id: recipe.output_item_id.to_string(), output_quantity: recipe.output_quantity, tool_id })
}

pub fn finish_production(job: &ProductionJob, station: &mut ProductionStation) -> Result<(), ProductionFailure> {
    if !job.job.complete() || !station.accepts(station.id) { return Err(ProductionFailure::OutputBlocked); }
    let remaining = station.store_output(&job.output_item_id, job.output_quantity, u32::MAX);
    if remaining != 0 { return Err(ProductionFailure::OutputBlocked); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CraftingIngredient, ProductionStationKind};
    const ING: &[CraftingIngredient] = &[CraftingIngredient { item_id: "wood", quantity: 2 }];
    const RECIPE: CraftingRecipe = CraftingRecipe { id:"test", ingredients:ING, output_item_id:"plank", output_quantity:1, station:Some("workbench"), required_technology:Some("construction"), duration_ticks:2 };
    #[test] fn start_is_atomic_for_validation_failures() { let mut i=Inventory::default(); i.add("wood",2,10); let s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); assert_eq!(start_production(&RECIPE,&mut i,&s,&[],None),Err(ProductionFailure::MissingTechnology)); assert_eq!(i.count("wood"),2); }
    #[test] fn production_consumes_and_finishes() { let mut i=Inventory::default(); i.add("wood",2,10); let s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); let mut j=start_production(&RECIPE,&mut i,&s,&["construction"],None).unwrap(); assert_eq!(i.count("wood"),0); j.job.tick(2); let mut s=s; assert!(finish_production(&j,&mut s).is_ok()); assert_eq!(s.output.count("plank"),1); }
    #[test] fn tool_is_consumed_once() { let mut i=Inventory::default(); i.add("wood",2,10); let s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); let mut t=ToolState::new("hammer",2).unwrap(); assert!(start_production(&RECIPE,&mut i,&s,&["construction"],Some(&mut t)).is_ok()); assert_eq!(t.durability,1); }
}

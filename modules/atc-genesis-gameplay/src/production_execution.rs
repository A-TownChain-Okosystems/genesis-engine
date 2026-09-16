//! Atomic deterministic production execution: validate, reserve, consume, queue, complete.

use crate::{CraftingJob, CraftingRecipe, Inventory, ProductionStation, ToolState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionFailure { InvalidRecipe, MissingIngredient, WrongStation, StationCapacity, MissingTechnology, MissingTool, ToolBroken, InputStorageBlocked, OutputBlocked, AlreadyCompleted }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionToolRequirement { pub tool_id: &'static str }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionJob {
    pub job: CraftingJob,
    pub station_id: &'static str,
    pub output_item_id: String,
    pub output_quantity: u32,
    pub tool_id: Option<String>,
    pub worker_id: Option<String>,
    pub output_committed: bool,
}

pub fn start_production(
    recipe: &CraftingRecipe,
    inventory: &mut Inventory,
    station: &mut ProductionStation,
    researched: &[&str],
    tool_requirement: Option<ProductionToolRequirement>,
    mut tool: Option<&mut ToolState>,
) -> Result<ProductionJob, ProductionFailure> {
    if recipe.id.is_empty() || recipe.output_item_id.is_empty() || recipe.output_quantity == 0 || recipe.duration_ticks == 0 || recipe.ingredients.iter().any(|i| i.item_id.is_empty() || i.quantity == 0) { return Err(ProductionFailure::InvalidRecipe); }
    if !station.accepts(&station.id) || recipe.station.map(|id| id != station.id).unwrap_or(false) { return Err(ProductionFailure::WrongStation); }
    if !station.has_job_capacity() { return Err(ProductionFailure::StationCapacity); }
    if let Some(tech) = recipe.required_technology { if !researched.contains(&tech) { return Err(ProductionFailure::MissingTechnology); } }
    if recipe.ingredients.iter().any(|i| inventory.count(i.item_id) < i.quantity) { return Err(ProductionFailure::MissingIngredient); }
    match (tool_requirement, tool.as_deref()) {
        (Some(req), Some(t)) if t.tool_id == req.tool_id && !t.broken() => {}
        (Some(_), Some(_)) | (Some(_), None) => return Err(ProductionFailure::MissingTool),
        (None, Some(t)) if t.broken() => return Err(ProductionFailure::ToolBroken),
        _ => {}
    }
    for ingredient in recipe.ingredients {
        if !station.input_has_storage(ingredient.item_id, ingredient.quantity, u32::MAX) { return Err(ProductionFailure::InputStorageBlocked); }
    }
    let job = CraftingJob::new(recipe).ok_or(ProductionFailure::InvalidRecipe)?;
    if !station.reserve_job() { return Err(ProductionFailure::StationCapacity); }
    let tool_id = tool.as_ref().map(|t| t.tool_id.to_string());
    if let Some(t) = tool.as_deref_mut() { if !t.use_once() { station.release_job(); return Err(ProductionFailure::ToolBroken); } }
    for ingredient in recipe.ingredients {
        if !inventory.remove(ingredient.item_id, ingredient.quantity) {
            station.release_job();
            return Err(ProductionFailure::MissingIngredient);
        }
    }
    Ok(ProductionJob { job, station_id: station.id, output_item_id: recipe.output_item_id.to_string(), output_quantity: recipe.output_quantity, tool_id, worker_id: None, output_committed: false })
}

pub fn finish_production(job: &mut ProductionJob, station: &mut ProductionStation) -> Result<(), ProductionFailure> {
    if job.output_committed { return Err(ProductionFailure::AlreadyCompleted); }
    if job.station_id != station.id || !job.job.complete() || !station.enabled { return Err(ProductionFailure::OutputBlocked); }
    if !station.output_has_storage(&job.output_item_id, job.output_quantity, u32::MAX) { return Err(ProductionFailure::OutputBlocked); }
    let remaining = station.store_output(&job.output_item_id, job.output_quantity, u32::MAX);
    if remaining != 0 { return Err(ProductionFailure::OutputBlocked); }
    job.output_committed = true;
    station.release_job();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CraftingIngredient, ProductionStationKind};
    const ING: &[CraftingIngredient] = &[CraftingIngredient { item_id: "wood", quantity: 2 }];
    const RECIPE: CraftingRecipe = CraftingRecipe { id:"test", ingredients:ING, output_item_id:"plank", output_quantity:1, station:Some("workbench"), required_technology:Some("construction"), duration_ticks:2 };
    #[test] fn validation_failure_does_not_mutate() { let mut i=Inventory::default(); i.add("wood",2,10); let mut s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); assert_eq!(start_production(&RECIPE,&mut i,&mut s,&[],None,None),Err(ProductionFailure::MissingTechnology)); assert_eq!(i.count("wood"),2); assert_eq!(s.active_jobs,0); }
    #[test] fn capacity_blocks_second_job() { let mut i=Inventory::default(); i.add("wood",4,10); let mut s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); assert!(start_production(&RECIPE,&mut i,&mut s,&["construction"],None,None).is_ok()); assert_eq!(start_production(&RECIPE,&mut i,&mut s,&["construction"],None,None),Err(ProductionFailure::StationCapacity)); }
    #[test] fn production_consumes_and_finishes_once() { let mut i=Inventory::default(); i.add("wood",2,10); let mut s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); let mut j=start_production(&RECIPE,&mut i,&mut s,&["construction"],None,None).unwrap(); assert_eq!(i.count("wood"),0); j.job.tick(2); assert!(finish_production(&mut j,&mut s).is_ok()); assert_eq!(s.output.count("plank"),1); assert_eq!(s.active_jobs,0); assert_eq!(finish_production(&mut j,&mut s),Err(ProductionFailure::AlreadyCompleted)); assert!(j.worker_id.is_none()); }
    #[test] fn output_capacity_blocks_without_job_release_or_mutation() { let mut i=Inventory::default(); i.add("wood",2,10); let mut s=ProductionStation::with_storage("workbench",ProductionStationKind::Workbench,1,1,1).unwrap(); s.output.add("plank",u32::MAX, u32::MAX); let mut j=start_production(&RECIPE,&mut i,&mut s,&["construction"],None,None).unwrap(); j.job.tick(2); assert_eq!(finish_production(&mut j,&mut s),Err(ProductionFailure::OutputBlocked)); assert!(!j.output_committed); assert_eq!(s.active_jobs,1); }
    #[test] fn required_tool_is_enforced_and_worn_once() { let mut i=Inventory::default(); i.add("wood",2,10); let mut s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); let mut t=ToolState::new("hammer",2).unwrap(); let req=ProductionToolRequirement{tool_id:"hammer"}; assert!(start_production(&RECIPE,&mut i,&mut s,&["construction"],Some(req),Some(&mut t)).is_ok()); assert_eq!(t.durability,1); }
    #[test] fn wrong_tool_does_not_mutate() { let mut i=Inventory::default(); i.add("wood",2,10); let mut s=ProductionStation::new("workbench",ProductionStationKind::Workbench,1).unwrap(); let mut t=ToolState::new("axe",2).unwrap(); let req=ProductionToolRequirement{tool_id:"hammer"}; assert_eq!(start_production(&RECIPE,&mut i,&mut s,&["construction"],Some(req),Some(&mut t)),Err(ProductionFailure::MissingTool)); assert_eq!(i.count("wood"),2); assert_eq!(t.durability,2); assert_eq!(s.active_jobs,0); }
}

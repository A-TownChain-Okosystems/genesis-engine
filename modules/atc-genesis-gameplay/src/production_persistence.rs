//! Deterministic persistence/resume state for production jobs.

use crate::ProductionJob;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionJobState {
    pub recipe_id: String,
    pub station_id: String,
    pub remaining_ticks: u64,
    pub total_ticks: u64,
    pub paused: bool,
    pub output_item_id: String,
    pub output_quantity: u32,
    pub tool_id: Option<String>,
    pub worker_id: Option<String>,
    pub output_committed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionPersistenceFailure {
    InvalidState,
    RecipeMismatch,
    StationMismatch,
    OutputMismatch,
    AlreadyCompleted,
}

impl ProductionJobState {
    pub fn capture(job: &ProductionJob) -> Self {
        Self {
            recipe_id: job.job.recipe_id.clone(),
            station_id: job.station_id.to_string(),
            remaining_ticks: job.job.remaining_ticks,
            total_ticks: job.job.total_ticks,
            paused: job.job.paused,
            output_item_id: job.output_item_id.clone(),
            output_quantity: job.output_quantity,
            tool_id: job.tool_id.clone(),
            worker_id: job.worker_id.clone(),
            output_committed: job.output_committed,
        }
    }

    pub fn validate(&self) -> bool {
        !self.recipe_id.is_empty()
            && !self.station_id.is_empty()
            && self.total_ticks > 0
            && self.remaining_ticks <= self.total_ticks
            && !self.output_item_id.is_empty()
            && self.output_quantity > 0
    }
}

pub fn restore_production_job(
    job: &mut ProductionJob,
    state: &ProductionJobState,
) -> Result<(), ProductionPersistenceFailure> {
    if !state.validate() { return Err(ProductionPersistenceFailure::InvalidState); }
    if job.job.recipe_id != state.recipe_id { return Err(ProductionPersistenceFailure::RecipeMismatch); }
    if job.station_id != state.station_id { return Err(ProductionPersistenceFailure::StationMismatch); }
    if job.output_item_id != state.output_item_id || job.output_quantity != state.output_quantity {
        return Err(ProductionPersistenceFailure::OutputMismatch);
    }
    if job.output_committed { return Err(ProductionPersistenceFailure::AlreadyCompleted); }
    job.job.remaining_ticks = state.remaining_ticks;
    job.job.total_ticks = state.total_ticks;
    job.job.paused = state.paused;
    job.tool_id = state.tool_id.clone();
    job.worker_id = state.worker_id.clone();
    job.output_committed = state.output_committed;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{start_production, CraftingIngredient, CraftingRecipe, Inventory, ProductionStation, ProductionStationKind};

    const ING: &[CraftingIngredient] = &[CraftingIngredient { item_id: "wood", quantity: 1 }];
    const RECIPE: CraftingRecipe = CraftingRecipe { id: "persist-test", ingredients: ING, output_item_id: "plank", output_quantity: 1, station: Some("workbench"), required_technology: None, duration_ticks: 20 };

    fn job() -> ProductionJob {
        let mut inventory = Inventory::default();
        inventory.add("wood", 1, 10);
        let mut station = ProductionStation::new("workbench", ProductionStationKind::Workbench, 1).unwrap();
        start_production(&RECIPE, &mut inventory, &mut station, &[], None, None).unwrap()
    }

    #[test]
    fn capture_and_restore_preserves_progress_and_pause_state() {
        let mut original = job();
        original.job.tick(7);
        original.job.pause();
        let state = ProductionJobState::capture(&original);
        let mut resumed = job();
        restore_production_job(&mut resumed, &state).unwrap();
        assert_eq!(resumed.job.remaining_ticks, 13);
        assert_eq!(resumed.job.total_ticks, 20);
        assert!(resumed.job.paused);
    }

    #[test]
    fn invalid_state_is_rejected_without_mutation() {
        let mut target = job();
        let before = target.clone();
        let mut state = ProductionJobState::capture(&target);
        state.remaining_ticks = state.total_ticks + 1;
        assert_eq!(restore_production_job(&mut target, &state), Err(ProductionPersistenceFailure::InvalidState));
        assert_eq!(target, before);
    }

    #[test]
    fn identity_mismatch_is_rejected() {
        let mut target = job();
        let mut state = ProductionJobState::capture(&target);
        state.recipe_id = "other".into();
        assert_eq!(restore_production_job(&mut target, &state), Err(ProductionPersistenceFailure::RecipeMismatch));
    }
}

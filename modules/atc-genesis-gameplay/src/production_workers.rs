//! Deterministic worker/NPC assignment for production jobs.

use crate::production_execution::ProductionJob;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionWorker {
    pub id: &'static str,
    pub skill_permille: u16,
    pub available: bool,
    pub assigned_station_id: Option<&'static str>,
}

impl ProductionWorker {
    pub fn new(id: &'static str, skill_permille: u16) -> Option<Self> {
        if id.is_empty() || skill_permille > 1000 { return None; }
        Some(Self { id, skill_permille, available: true, assigned_station_id: None })
    }

    pub fn assign(&mut self, station_id: &'static str) -> bool {
        if !self.available || station_id.is_empty() { return false; }
        self.available = false;
        self.assigned_station_id = Some(station_id);
        true
    }

    pub fn release(&mut self) -> bool {
        if self.available && self.assigned_station_id.is_none() { return false; }
        self.available = true;
        self.assigned_station_id = None;
        true
    }

    pub fn effective_duration(&self, base_ticks: u64) -> u64 {
        if base_ticks == 0 { return 0; }
        let skill = self.skill_permille.max(1) as u64;
        ((base_ticks.saturating_mul(1000) + skill - 1) / skill).max(1)
    }
}

pub fn assign_worker(job: &mut ProductionJob, worker: &mut ProductionWorker) -> bool {
    if job.worker_id.is_some() || !worker.assign(job.station_id) { return false; }
    job.worker_id = Some(worker.id.to_string());
    true
}

pub fn release_worker(job: &mut ProductionJob, worker: &mut ProductionWorker) -> bool {
    if job.worker_id.as_deref() != Some(worker.id) { return false; }
    job.worker_id = None;
    worker.release()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CraftingIngredient, CraftingRecipe, Inventory, ProductionStation, ProductionStationKind, start_production};

    const ING: &[CraftingIngredient] = &[CraftingIngredient { item_id: "wood", quantity: 1 }];
    const RECIPE: CraftingRecipe = CraftingRecipe { id: "worker-test", ingredients: ING, output_item_id: "plank", output_quantity: 1, station: Some("workbench"), required_technology: None, duration_ticks: 10 };

    #[test]
    fn worker_assignment_is_exclusive_and_releasable() {
        let mut inventory = Inventory::default();
        inventory.add("wood", 1, 10);
        let mut station = ProductionStation::new("workbench", ProductionStationKind::Workbench, 1).unwrap();
        let mut job = start_production(&RECIPE, &mut inventory, &mut station, &[], None, None).unwrap();
        let mut worker = ProductionWorker::new("worker-1", 1000).unwrap();
        assert!(assign_worker(&mut job, &mut worker));
        assert!(!assign_worker(&mut job, &mut worker));
        assert!(!worker.available);
        assert!(release_worker(&mut job, &mut worker));
        assert!(worker.available);
        assert!(job.worker_id.is_none());
    }

    #[test]
    fn skill_duration_is_deterministic_and_bounded() {
        let worker = ProductionWorker::new("worker-1", 500).unwrap();
        assert_eq!(worker.effective_duration(10), 20);
        assert_eq!(worker.effective_duration(0), 0);
    }

    #[test]
    fn invalid_worker_is_rejected() {
        assert!(ProductionWorker::new("", 100).is_none());
        assert!(ProductionWorker::new("worker", 1001).is_none());
    }
}

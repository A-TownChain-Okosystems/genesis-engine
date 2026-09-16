//! Deterministic crash-safe in-memory journal for production snapshots.
//!
//! The store provides versioned records and an explicit commit marker. It is
//! storage-backend neutral: callers can persist the journal bytes externally.

use crate::{ProductionJob, ProductionJobState};

pub const PRODUCTION_STATE_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionRecord {
    pub version: u16,
    pub sequence: u64,
    pub committed: bool,
    pub state: ProductionJobState,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProductionJournal {
    records: Vec<ProductionRecord>,
    next_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionStoreFailure {
    InvalidVersion,
    InvalidState,
    Uncommitted,
    EmptyJournal,
    SequenceConflict,
}

impl ProductionJournal {
    pub fn append_snapshot(&mut self, job: &ProductionJob) -> Result<u64, ProductionStoreFailure> {
        let state = ProductionJobState::capture(job);
        if !state.validate() { return Err(ProductionStoreFailure::InvalidState); }
        let sequence = self.next_sequence.saturating_add(1);
        self.next_sequence = sequence;
        self.records.push(ProductionRecord {
            version: PRODUCTION_STATE_VERSION,
            sequence,
            committed: false,
            state,
        });
        Ok(sequence)
    }

    pub fn commit(&mut self, sequence: u64) -> Result<(), ProductionStoreFailure> {
        let Some(record) = self.records.iter_mut().find(|r| r.sequence == sequence) else {
            return Err(ProductionStoreFailure::SequenceConflict);
        };
        if record.version != PRODUCTION_STATE_VERSION { return Err(ProductionStoreFailure::InvalidVersion); }
        record.committed = true;
        Ok(())
    }

    pub fn latest_committed(&self) -> Result<&ProductionRecord, ProductionStoreFailure> {
        self.records.iter()
            .rev()
            .find(|r| r.committed && r.version == PRODUCTION_STATE_VERSION)
            .ok_or(ProductionStoreFailure::EmptyJournal)
    }

    pub fn recover(&self, job: &mut ProductionJob) -> Result<(), ProductionStoreFailure> {
        let record = self.latest_committed()?;
        if !record.state.validate() { return Err(ProductionStoreFailure::InvalidState); }
        crate::restore_production_job(job, &record.state)
            .map_err(|_| ProductionStoreFailure::InvalidState)
    }

    pub fn records(&self) -> &[ProductionRecord] { &self.records }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{start_production, CraftingIngredient, CraftingRecipe, Inventory, ProductionStation, ProductionStationKind};

    const ING: &[CraftingIngredient] = &[CraftingIngredient { item_id: "wood", quantity: 1 }];
    const RECIPE: CraftingRecipe = CraftingRecipe { id: "journal-test", ingredients: ING, output_item_id: "plank", output_quantity: 1, station: Some("workbench"), required_technology: None, duration_ticks: 20 };

    fn job() -> ProductionJob {
        let mut inventory = Inventory::default();
        inventory.add("wood", 1, 10);
        let mut station = ProductionStation::new("workbench", ProductionStationKind::Workbench, 1).unwrap();
        start_production(&RECIPE, &mut inventory, &mut station, &[], None, None).unwrap()
    }

    #[test]
    fn uncommitted_snapshot_is_not_recovered() {
        let mut journal = ProductionJournal::default();
        let mut source = job();
        source.job.tick(5);
        journal.append_snapshot(&source).unwrap();
        let mut target = job();
        assert_eq!(journal.recover(&mut target), Err(ProductionStoreFailure::EmptyJournal));
    }

    #[test]
    fn committed_snapshot_recovers_deterministically() {
        let mut journal = ProductionJournal::default();
        let mut source = job();
        source.job.tick(7);
        let sequence = journal.append_snapshot(&source).unwrap();
        journal.commit(sequence).unwrap();
        let mut target = job();
        journal.recover(&mut target).unwrap();
        assert_eq!(target.job.remaining_ticks, 13);
        assert_eq!(journal.latest_committed().unwrap().sequence, 1);
    }

    #[test]
    fn sequence_numbers_are_monotonic() {
        let mut journal = ProductionJournal::default();
        let job = job();
        let a = journal.append_snapshot(&job).unwrap();
        let b = journal.append_snapshot(&job).unwrap();
        assert_eq!((a, b), (1, 2));
    }
}

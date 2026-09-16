//! Deterministic timed crafting jobs and queues.

use crate::crafting::CraftingRecipe;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CraftingJob {
    pub recipe_id: String,
    pub remaining_ticks: u64,
    pub total_ticks: u64,
    pub paused: bool,
}

impl CraftingJob {
    pub fn new(recipe: &CraftingRecipe) -> Option<Self> {
        if recipe.id.is_empty() || recipe.duration_ticks == 0 { return None; }
        Some(Self { recipe_id: recipe.id.to_string(), remaining_ticks: recipe.duration_ticks as u64, total_ticks: recipe.duration_ticks as u64, paused: false })
    }
    pub fn tick(&mut self, ticks: u64) {
        if self.paused { return; }
        self.remaining_ticks = self.remaining_ticks.saturating_sub(ticks);
    }
    pub fn pause(&mut self) { self.paused = true; }
    pub fn resume(&mut self) { self.paused = false; }
    pub fn complete(&self) -> bool { self.remaining_ticks == 0 }
    pub fn progress_permille(&self) -> u16 {
        if self.total_ticks == 0 { return 0; }
        (((self.total_ticks - self.remaining_ticks).saturating_mul(1000) / self.total_ticks).min(1000)) as u16
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CraftingQueue { pub jobs: Vec<CraftingJob> }

impl CraftingQueue {
    pub fn enqueue(&mut self, job: CraftingJob) { self.jobs.push(job); }
    pub fn tick(&mut self, ticks: u64) { for job in &mut self.jobs { job.tick(ticks); } }
    pub fn completed(&self) -> impl Iterator<Item=&CraftingJob> { self.jobs.iter().filter(|j| j.complete()) }
    pub fn remove_completed(&mut self) -> usize {
        let before = self.jobs.len();
        self.jobs.retain(|j| !j.complete());
        before - self.jobs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crafting::WORKBENCH_RECIPE;
    #[test] fn job_progresses_deterministically() { let mut j=CraftingJob::new(&WORKBENCH_RECIPE).unwrap(); j.tick(25); assert_eq!(j.progress_permille(),500); assert!(!j.complete()); }
    #[test] fn paused_job_does_not_progress() { let mut j=CraftingJob::new(&WORKBENCH_RECIPE).unwrap(); j.pause(); j.tick(100); assert_eq!(j.remaining_ticks,50); j.resume(); j.tick(50); assert!(j.complete()); }
    #[test] fn queue_removes_finished_jobs() { let mut q=CraftingQueue::default(); q.enqueue(CraftingJob::new(&WORKBENCH_RECIPE).unwrap()); q.tick(50); assert_eq!(q.remove_completed(),1); assert!(q.jobs.is_empty()); }
}

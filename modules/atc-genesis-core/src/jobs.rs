use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JobId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobError { QueueFull, IdOverflow }

pub struct JobQueue<T> { next_id: u64, capacity: usize, queue: VecDeque<(JobId, T)> }
impl<T> JobQueue<T> {
    pub fn new(capacity: usize) -> Self { Self { next_id: 0, capacity, queue: VecDeque::new() } }
    pub fn push(&mut self, job: T) -> Result<JobId, JobError> {
        if self.queue.len() >= self.capacity { return Err(JobError::QueueFull); }
        let id = JobId(self.next_id);
        self.next_id = self.next_id.checked_add(1).ok_or(JobError::IdOverflow)?;
        self.queue.push_back((id, job));
        Ok(id)
    }
    pub fn pop(&mut self) -> Option<(JobId, T)> { self.queue.pop_front() }
    pub fn len(&self) -> usize { self.queue.len() }
    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fifo_is_deterministic() {
        let mut q = JobQueue::new(2);
        let a = q.push("a").unwrap(); let b = q.push("b").unwrap();
        assert_eq!(q.pop(), Some((a, "a"))); assert_eq!(q.pop(), Some((b, "b")));
    }
    #[test] fn capacity_is_enforced() { let mut q = JobQueue::new(1); q.push(1).unwrap(); assert_eq!(q.push(2), Err(JobError::QueueFull)); }
}

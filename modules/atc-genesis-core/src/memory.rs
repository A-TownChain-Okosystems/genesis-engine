use std::collections::VecDeque;
use super::GenerationalId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Allocation { pub offset: usize, pub size: usize }

#[derive(Debug)]
pub struct LinearAllocator { capacity: usize, cursor: usize }
impl LinearAllocator {
    pub fn new(capacity: usize) -> Self { Self { capacity, cursor: 0 } }
    pub fn allocate(&mut self, size: usize, alignment: usize) -> Option<Allocation> {
        if alignment == 0 || !alignment.is_power_of_two() { return None; }
        let mask = alignment - 1;
        let offset = self.cursor.checked_add(mask)?.checked_sub(self.cursor.checked_add(mask)? & mask)?;
        let end = offset.checked_add(size)?;
        if end > self.capacity { return None; }
        self.cursor = end;
        Some(Allocation { offset, size })
    }
    pub fn used(&self) -> usize { self.cursor }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn reset(&mut self) { self.cursor = 0; }
}

pub struct GenerationalStorage<T> { entries: Vec<Option<(u32, T)>>, free: VecDeque<u32> }
impl<T> Default for GenerationalStorage<T> { fn default() -> Self { Self { entries: Vec::new(), free: VecDeque::new() } } }
impl<T> GenerationalStorage<T> {
    pub fn insert(&mut self, value: T) -> Option<GenerationalId> {
        if let Some(index) = self.free.pop_front() {
            let slot = self.entries.get_mut(index as usize)?;
            let generation = slot.as_ref().map_or(1, |x| x.0.saturating_add(1));
            *slot = Some((generation, value));
            return Some(GenerationalId::new(index, generation));
        }
        let index = u32::try_from(self.entries.len()).ok()?;
        self.entries.push(Some((1, value)));
        Some(GenerationalId::new(index, 1))
    }
    pub fn get(&self, id: GenerationalId) -> Option<&T> { self.entries.get(id.index as usize)?.as_ref().filter(|x| x.0 == id.generation).map(|x| &x.1) }
    pub fn get_mut(&mut self, id: GenerationalId) -> Option<&mut T> { self.entries.get_mut(id.index as usize)?.as_mut().filter(|x| x.0 == id.generation).map(|x| &mut x.1) }
    pub fn remove(&mut self, id: GenerationalId) -> Option<T> {
        let slot = self.entries.get_mut(id.index as usize)?;
        if slot.as_ref()?.0 != id.generation { return None; }
        let value = slot.take()?.1;
        self.free.push_back(id.index);
        Some(value)
    }
    pub fn len(&self) -> usize { self.entries.iter().filter(|x| x.is_some()).count() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn allocator_aligns_and_resets() { let mut a=LinearAllocator::new(64); assert_eq!(a.allocate(3,8),Some(Allocation{offset:0,size:3})); assert_eq!(a.allocate(4,8),Some(Allocation{offset:8,size:4})); a.reset(); assert_eq!(a.used(),0); }
    #[test] fn storage_rejects_stale_handles() { let mut s=GenerationalStorage::default(); let a=s.insert(10).unwrap(); assert_eq!(s.remove(a),Some(10)); assert!(s.get(a).is_none()); let b=s.insert(20).unwrap(); assert_ne!(a,b); assert_eq!(s.get(b),Some(&20)); }
}

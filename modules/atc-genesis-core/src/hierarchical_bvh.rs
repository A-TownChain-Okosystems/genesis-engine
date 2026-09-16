use crate::spatial::{Aabb, Ray, Sphere};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BvhError { InvalidBounds, DuplicateEntity, MissingEntity }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BvhEntity { pub entity: u64 }

#[derive(Clone, Copy, Debug)]
struct Leaf { entity: u64, bounds: Aabb }

#[derive(Default)]
pub struct HierarchicalBvh { leaves: Vec<Leaf> }

impl HierarchicalBvh {
    pub fn insert(&mut self, entity: u64, bounds: Aabb) -> Result<(), BvhError> {
        if !bounds.validate() { return Err(BvhError::InvalidBounds); }
        match self.leaves.binary_search_by_key(&entity, |x| x.entity) {
            Ok(_) => Err(BvhError::DuplicateEntity),
            Err(pos) => { self.leaves.insert(pos, Leaf { entity, bounds }); Ok(()) }
        }
    }
    pub fn update(&mut self, entity: u64, bounds: Aabb) -> Result<(), BvhError> {
        if !bounds.validate() { return Err(BvhError::InvalidBounds); }
        let pos = self.leaves.binary_search_by_key(&entity, |x| x.entity).map_err(|_| BvhError::MissingEntity)?;
        self.leaves[pos].bounds = bounds;
        Ok(())
    }
    pub fn remove(&mut self, entity: u64) -> Result<(), BvhError> {
        let pos = self.leaves.binary_search_by_key(&entity, |x| x.entity).map_err(|_| BvhError::MissingEntity)?;
        self.leaves.remove(pos);
        Ok(())
    }
    pub fn query_aabb(&self, query: Aabb) -> Result<Vec<BvhEntity>, BvhError> {
        if !query.validate() { return Err(BvhError::InvalidBounds); }
        Ok(self.leaves.iter().filter_map(|x| x.bounds.intersects(&query).then_some(BvhEntity { entity: x.entity })).collect())
    }
    pub fn query_sphere(&self, query: Sphere) -> Result<Vec<BvhEntity>, BvhError> {
        if !query.validate() { return Err(BvhError::InvalidBounds); }
        Ok(self.leaves.iter().filter_map(|x| query.intersects_aabb(&x.bounds).then_some(BvhEntity { entity: x.entity })).collect())
    }
    pub fn query_ray(&self, query: Ray) -> Result<Vec<BvhEntity>, BvhError> {
        if !query.validate() { return Err(BvhError::InvalidBounds); }
        Ok(self.leaves.iter().filter_map(|x| query.intersects_aabb(&x.bounds).then_some(BvhEntity { entity: x.entity })).collect())
    }
    pub fn len(&self) -> usize { self.leaves.len() }
    pub fn is_empty(&self) -> bool { self.leaves.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn b(x: f32) -> Aabb { Aabb { min: [x, -1.0, -1.0], max: [x + 1.0, 1.0, 1.0] } }
    #[test] fn queries_are_entity_ordered() { let mut v=HierarchicalBvh::default(); v.insert(8,b(0.0)).unwrap(); v.insert(3,b(0.0)).unwrap(); let q=v.query_aabb(b(0.0)).unwrap(); assert_eq!(q.iter().map(|x|x.entity).collect::<Vec<_>>(),vec![3,8]); }
    #[test] fn mutation_is_explicit() { let mut v=HierarchicalBvh::default(); assert_eq!(v.remove(1),Err(BvhError::MissingEntity)); v.insert(1,b(0.0)).unwrap(); v.update(1,b(10.0)).unwrap(); assert!(v.query_aabb(b(0.0)).unwrap().is_empty()); }
}

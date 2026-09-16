use crate::spatial::Aabb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DynamicLeaf { pub entity:u64 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DynamicBvhError { InvalidBounds, DuplicateEntity, MissingEntity }

#[derive(Default)]
pub struct DynamicBvh { leaves: Vec<(DynamicLeaf,Aabb)> }
impl DynamicBvh {
    pub fn insert(&mut self, entity:u64, bounds:Aabb)->Result<(),DynamicBvhError>{
        if !bounds.validate(){return Err(DynamicBvhError::InvalidBounds)}
        match self.leaves.binary_search_by_key(&entity, |(leaf,_)| leaf.entity) {
            Ok(_) => Err(DynamicBvhError::DuplicateEntity),
            Err(pos) => { self.leaves.insert(pos,(DynamicLeaf{entity},bounds)); Ok(()) }
        }
    }
    pub fn update(&mut self, entity:u64, bounds:Aabb)->Result<(),DynamicBvhError>{
        if !bounds.validate(){return Err(DynamicBvhError::InvalidBounds)}
        let pos=self.leaves.binary_search_by_key(&entity, |(leaf,_)| leaf.entity).map_err(|_|DynamicBvhError::MissingEntity)?;
        self.leaves[pos].1=bounds; Ok(())
    }
    pub fn remove(&mut self, entity:u64)->Result<(),DynamicBvhError>{
        let pos=self.leaves.binary_search_by_key(&entity, |(leaf,_)| leaf.entity).map_err(|_|DynamicBvhError::MissingEntity)?;
        self.leaves.remove(pos); Ok(())
    }
    pub fn query_aabb(&self, query:Aabb)->Result<Vec<u64>,DynamicBvhError>{
        if !query.validate(){return Err(DynamicBvhError::InvalidBounds)}
        Ok(self.leaves.iter().filter_map(|(leaf,b)| b.intersects(&query).then_some(leaf.entity)).collect())
    }
    pub fn len(&self)->usize{self.leaves.len()}
    pub fn is_empty(&self)->bool{self.leaves.is_empty()}
}

#[cfg(test)]
mod tests {
 use super::*;
 fn bounds(x:f32)->Aabb{Aabb{min:[x,0.0,0.0],max:[x+1.0,1.0,1.0]}}
 #[test] fn ordering_is_deterministic(){let mut b=DynamicBvh::default();b.insert(9,bounds(0.0)).unwrap();b.insert(2,bounds(0.0)).unwrap();assert_eq!(b.query_aabb(bounds(0.0)).unwrap(),vec![2,9]);}
 #[test] fn update_and_remove(){let mut b=DynamicBvh::default();b.insert(1,bounds(0.0)).unwrap();b.update(1,bounds(10.0)).unwrap();assert!(b.query_aabb(bounds(0.0)).unwrap().is_empty());b.remove(1).unwrap();assert!(b.is_empty());}
}

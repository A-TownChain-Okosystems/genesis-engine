use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min:[f32;3], pub max:[f32;3] }
impl Aabb {
    pub fn validate(&self)->bool { self.min.iter().chain(self.max.iter()).all(|v|v.is_finite()) && (0..3).all(|i| self.min[i] <= self.max[i]) }
    pub fn intersects(&self, other:&Self)->bool { self.validate() && other.validate() && (0..3).all(|i| self.min[i] <= other.max[i] && self.max[i] >= other.min[i]) }
    pub fn contains_point(&self,p:[f32;3])->bool { self.validate() && p.iter().all(|v|v.is_finite()) && (0..3).all(|i| p[i]>=self.min[i] && p[i]<=self.max[i]) }
    pub fn center(&self)->[f32;3]{[(self.min[0]+self.max[0])*0.5,(self.min[1]+self.max[1])*0.5,(self.min[2]+self.max[2])*0.5]}
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere { pub center:[f32;3], pub radius:f32 }
impl Sphere { pub fn validate(&self)->bool{self.center.iter().all(|v|v.is_finite())&&self.radius.is_finite()&&self.radius>=0.0} pub fn intersects_aabb(&self,b:&Aabb)->bool{if !self.validate()||!b.validate(){return false} let mut d=0.0;for i in 0..3{let q=self.center[i].clamp(b.min[i],b.max[i]);let x=self.center[i]-q;d+=x*x}d<=self.radius*self.radius} }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray { pub origin:[f32;3], pub direction:[f32;3] }
impl Ray { pub fn validate(&self)->bool{self.origin.iter().chain(self.direction.iter()).all(|v|v.is_finite())&&self.direction.iter().any(|v|v.abs()>f32::EPSILON)} pub fn intersects_aabb(&self,b:&Aabb)->bool{if !self.validate()||!b.validate(){return false} let mut tmin=0.0;let mut tmax=f32::INFINITY;for i in 0..3{if self.direction[i].abs()<=f32::EPSILON{if self.origin[i]<b.min[i]||self.origin[i]>b.max[i]{return false}}else{let inv=1.0/self.direction[i];let mut t1=(b.min[i]-self.origin[i])*inv;let mut t2=(b.max[i]-self.origin[i])*inv;if t1>t2{std::mem::swap(&mut t1,&mut t2)}tmin=tmin.max(t1);tmax=tmax.min(t2);if tmin>tmax{return false}}}true} }

#[derive(Default)]
pub struct SpatialWorld { bounds:BTreeMap<u64,Aabb> }
impl SpatialWorld {
    pub fn insert(&mut self,entity:u64,bounds:Aabb)->Result<(),SpatialError>{if !bounds.validate(){return Err(SpatialError::InvalidBounds)} if self.bounds.insert(entity,bounds).is_some(){return Err(SpatialError::DuplicateEntity)} Ok(())}
    pub fn update(&mut self,entity:u64,bounds:Aabb)->Result<(),SpatialError>{if !bounds.validate(){return Err(SpatialError::InvalidBounds)} if !self.bounds.contains_key(&entity){return Err(SpatialError::MissingEntity)} self.bounds.insert(entity,bounds);Ok(())}
    pub fn remove(&mut self,entity:u64)->bool{self.bounds.remove(&entity).is_some()}
    pub fn query_aabb(&self,query:Aabb)->Result<Vec<u64>,SpatialError>{if !query.validate(){return Err(SpatialError::InvalidBounds)} Ok(self.bounds.iter().filter_map(|(id,b)|b.intersects(&query).then_some(*id)).collect())}
    pub fn query_sphere(&self,query:Sphere)->Result<Vec<u64>,SpatialError>{if !query.validate(){return Err(SpatialError::InvalidSphere)} Ok(self.bounds.iter().filter_map(|(id,b)|query.intersects_aabb(b).then_some(*id)).collect())}
    pub fn query_ray(&self,query:Ray)->Result<Vec<u64>,SpatialError>{if !query.validate(){return Err(SpatialError::InvalidRay)} Ok(self.bounds.iter().filter_map(|(id,b)|query.intersects_aabb(b).then_some(*id)).collect())}
    pub fn len(&self)->usize{self.bounds.len()}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SpatialError{InvalidBounds,InvalidSphere,InvalidRay,DuplicateEntity,MissingEntity}

#[cfg(test)]mod tests{use super::*;fn b()->Aabb{Aabb{min:[-1.0,-1.0,-1.0],max:[1.0,1.0,1.0]}}#[test]fn deterministic_queries(){let mut s=SpatialWorld::default();s.insert(2,b()).unwrap();s.insert(1,Aabb{min:[4.0,4.0,4.0],max:[5.0,5.0,5.0]}).unwrap();assert_eq!(s.query_aabb(b()).unwrap(),vec![2]);}#[test]fn sphere_query(){let mut s=SpatialWorld::default();s.insert(7,b()).unwrap();assert_eq!(s.query_sphere(Sphere{center:[2.0,0.0,0.0],radius:1.0}).unwrap(),vec![7]);}#[test]fn ray_query(){let mut s=SpatialWorld::default();s.insert(4,b()).unwrap();assert_eq!(s.query_ray(Ray{origin:[-5.0,0.0,0.0],direction:[1.0,0.0,0.0]}).unwrap(),vec![4]);}}

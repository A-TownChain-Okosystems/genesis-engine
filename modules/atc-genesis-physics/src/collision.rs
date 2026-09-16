use atc_genesis_platform::EntityId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min: [f32; 3], pub max: [f32; 3] }

impl Aabb {
    pub fn validate(&self) -> bool {
        self.min.iter().chain(self.max.iter()).all(|v| v.is_finite())
            && (0..3).all(|i| self.min[i] <= self.max[i])
    }
    pub fn contains(&self, p: [f32; 3]) -> bool { self.validate() && p.iter().all(|v| v.is_finite()) && (0..3).all(|i| p[i] >= self.min[i] && p[i] <= self.max[i]) }
    pub fn intersects(&self, other: &Self) -> bool {
        self.validate() && other.validate() && (0..3).all(|i| self.min[i] <= other.max[i] && self.max[i] >= other.min[i])
    }
    pub fn ray_intersection(&self, origin: [f32; 3], direction: [f32; 3], max_distance: f32) -> Option<f32> {
        if !self.validate() || !origin.iter().chain(direction.iter()).all(|v| v.is_finite()) || max_distance < 0.0 { return None; }
        let mut near = 0.0f32; let mut far = max_distance;
        for i in 0..3 {
            if direction[i].abs() < f32::EPSILON { if origin[i] < self.min[i] || origin[i] > self.max[i] { return None; } continue; }
            let inv = 1.0 / direction[i]; let mut t1 = (self.min[i] - origin[i]) * inv; let mut t2 = (self.max[i] - origin[i]) * inv;
            if t1 > t2 { std::mem::swap(&mut t1, &mut t2); }
            near = near.max(t1); far = far.min(t2); if near > far { return None; }
        }
        Some(near)
    }
    pub fn point_resolution(&self, point: [f32; 3]) -> Option<([f32; 3], f32)> {
        if !self.contains(point) { return None; }
        let distances = [point[0]-self.min[0], self.max[0]-point[0], point[1]-self.min[1], self.max[1]-point[1], point[2]-self.min[2], self.max[2]-point[2]];
        let mut best=0usize; for i in 1..distances.len(){if distances[i] < distances[best]{best=i;}}
        let normal=match best{0=>[-1.0,0.0,0.0],1=>[1.0,0.0,0.0],2=>[0.0,-1.0,0.0],3=>[0.0,1.0,0.0],4=>[0.0,0.0,-1.0],_=>[0.0,0.0,1.0]}; Some((normal,distances[best]))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)] pub struct Collider { pub entity: EntityId, pub bounds: Aabb }
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)] pub struct CollisionPair { pub a: EntityId, pub b: EntityId }
impl CollisionPair { pub fn new(a: EntityId, b: EntityId) -> Option<Self> { if a==b {None} else if a.0<b.0 {Some(Self{a,b})} else {Some(Self{a:b,b:a})} } }

#[derive(Default)] pub struct CollisionWorld { colliders: Vec<Collider> }
impl CollisionWorld {
    pub fn add(&mut self, collider: Collider) { if let Some(i)=self.colliders.iter().position(|c| c.entity==collider.entity){self.colliders[i]=collider;}else{self.colliders.push(collider);} self.colliders.sort_by_key(|c|c.entity.0); }
    pub fn remove(&mut self, entity: EntityId) -> bool { if let Some(i)=self.colliders.iter().position(|c| c.entity==entity){self.colliders.remove(i);true}else{false} }
    pub fn query_aabb(&self, bounds:Aabb)->Vec<EntityId>{self.colliders.iter().filter(|c|c.bounds.intersects(&bounds)).map(|c|c.entity).collect()}
    pub fn broad_phase_pairs(&self)->Vec<CollisionPair>{let mut pairs=Vec::new();for i in 0..self.colliders.len(){for j in (i+1)..self.colliders.len(){if let Some(pair)=CollisionPair::new(self.colliders[i].entity,self.colliders[j].entity){if self.colliders[i].bounds.intersects(&self.colliders[j].bounds){pairs.push(pair);}}}}pairs}
    pub fn resolve_point(&self, point:[f32;3])->Option<(EntityId,[f32;3],f32)>{self.colliders.iter().filter_map(|c|c.bounds.point_resolution(point).map(|(n,d)|(c.entity,n,d))).min_by(|a,b|a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal).then_with(||a.0.0.cmp(&b.0.0)))}
    pub fn raycast(&self,origin:[f32;3],direction:[f32;3],max_distance:f32)->Option<EntityId>{self.colliders.iter().filter_map(|c|c.bounds.ray_intersection(origin,direction,max_distance).map(|t|(t,c.entity))).min_by(|a,b|a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal).then_with(||a.1.0.cmp(&b.1.0))).map(|(_,id)|id)}
    pub fn clear(&mut self){self.colliders.clear()} pub fn len(&self)->usize{self.colliders.len()}
}

#[cfg(test)]mod tests{use super::*;fn bounds(x:f32)->Aabb{Aabb{min:[x,-1.0,-1.0],max:[x+1.0,1.0,1.0]}}#[test]fn raycast_returns_nearest(){let mut w=CollisionWorld::default();w.add(Collider{entity:EntityId(2),bounds:bounds(5.0)});w.add(Collider{entity:EntityId(1),bounds:bounds(2.0)});assert_eq!(w.raycast([0.0,0.0,0.0],[1.0,0.0,0.0],10.0),Some(EntityId(1)));}#[test]fn broad_phase_pairs_are_canonical(){let mut w=CollisionWorld::default();w.add(Collider{entity:EntityId(2),bounds:bounds(0.0)});w.add(Collider{entity:EntityId(1),bounds:bounds(0.5)});w.add(Collider{entity:EntityId(3),bounds:bounds(10.0)});assert_eq!(w.broad_phase_pairs(),vec![CollisionPair{a:EntityId(1),b:EntityId(2)}]);}#[test]fn point_resolution_is_deterministic(){let mut w=CollisionWorld::default();let b=bounds(0.0);w.add(Collider{entity:EntityId(2),bounds:b});w.add(Collider{entity:EntityId(1),bounds:b});assert_eq!(w.resolve_point([0.0,0.0,0.0]).unwrap().0,EntityId(1));}}
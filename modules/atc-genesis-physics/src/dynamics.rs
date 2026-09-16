#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 { pub x:f32,pub y:f32,pub z:f32 }
impl Vec3 { pub const ZERO:Self=Self{x:0.0,y:0.0,z:0.0}; pub fn add(self,o:Self)->Self{Self{x:self.x+o.x,y:self.y+o.y,z:self.z+o.z}} pub fn scale(self,s:f32)->Self{Self{x:self.x*s,y:self.y*s,z:self.z*s}} pub fn is_finite(self)->bool{self.x.is_finite()&&self.y.is_finite()&&self.z.is_finite()} }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyType { Static, Dynamic, Kinematic }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody { pub id:u64,pub body_type:BodyType,pub position:Vec3,pub velocity:Vec3,pub force:Vec3,pub mass:f32 }
impl RigidBody { pub fn validate(&self)->bool{self.id!=0&&self.position.is_finite()&&self.velocity.is_finite()&&self.force.is_finite()&&self.mass.is_finite()&&self.mass>0.0} }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicsError { InvalidBody, DuplicateBody, MissingBody, InvalidDelta }
#[derive(Default)]
pub struct PhysicsWorld { bodies:Vec<RigidBody> }
impl PhysicsWorld {
 pub fn insert(&mut self,body:RigidBody)->Result<(),PhysicsError>{if !body.validate(){return Err(PhysicsError::InvalidBody)}if self.bodies.binary_search_by_key(&body.id,|b|b.id).is_ok(){return Err(PhysicsError::DuplicateBody)}let p=self.bodies.binary_search_by_key(&body.id,|b|b.id).unwrap_or_else(|p|p);self.bodies.insert(p,body);Ok(())}
 pub fn get(&self,id:u64)->Option<&RigidBody>{self.bodies.binary_search_by_key(&id,|b|b.id).ok().map(|i|&self.bodies[i])}
 pub fn step(&mut self,delta:f32)->Result<(),PhysicsError>{if !delta.is_finite()||delta<0.0{return Err(PhysicsError::InvalidDelta)}for body in &mut self.bodies{match body.body_type{BodyType::Static=>{},BodyType::Kinematic=>{body.position=body.position.add(body.velocity.scale(delta));},BodyType::Dynamic=>{let acceleration=body.force.scale(1.0/body.mass);body.velocity=body.velocity.add(acceleration.scale(delta));body.position=body.position.add(body.velocity.scale(delta));}}}Ok(())}
 pub fn clear_forces(&mut self){for b in &mut self.bodies{b.force=Vec3::ZERO;}}
 pub fn len(&self)->usize{self.bodies.len()}
 pub fn iter(&self)->impl Iterator<Item=&RigidBody>{self.bodies.iter()}
}
#[cfg(test)]mod tests{use super::*;fn body(id:u64)->RigidBody{RigidBody{id,body_type:BodyType::Dynamic,position:Vec3::ZERO,velocity:Vec3::ZERO,force:Vec3{x:10.0,y:0.0,z:0.0},mass:2.0}}#[test]fn integration_is_deterministic(){let mut w=PhysicsWorld::default();w.insert(body(2)).unwrap();w.insert(body(1)).unwrap();w.step(1.0).unwrap();assert_eq!(w.get(1).unwrap().velocity.x,5.0);assert_eq!(w.get(1).unwrap().position.x,5.0);}#[test]fn duplicate_is_rejected(){let mut w=PhysicsWorld::default();w.insert(body(1)).unwrap();assert_eq!(w.insert(body(1)),Err(PhysicsError::DuplicateBody));}#[test]fn static_does_not_move(){let mut w=PhysicsWorld::default();let mut b=body(1);b.body_type=BodyType::Static;w.insert(b).unwrap();w.step(1.0).unwrap();assert_eq!(w.get(1).unwrap().position,Vec3::ZERO);}}

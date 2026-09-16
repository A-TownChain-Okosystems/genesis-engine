use super::frustum::Frustum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingSphere { pub center: [f32; 3], pub radius: f32 }
impl BoundingSphere {
    pub fn validate(&self)->bool { self.center.iter().all(|v|v.is_finite())&&self.radius.is_finite()&&self.radius>=0.0 }
    pub fn intersects_frustum(&self,frustum:&Frustum)->bool { self.validate()&&frustum.planes.iter().all(|p|p.signed_distance(self.center)>=-self.radius) }
    pub fn translated(&self,t:[f32;3])->Option<Self> { if !t.iter().all(|v|v.is_finite()){return None} Some(Self{center:[self.center[0]+t[0],self.center[1]+t[1],self.center[2]+t[2]],radius:self.radius}) }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min:[f32;3], pub max:[f32;3] }
impl Aabb {
    pub fn validate(&self)->bool { self.min.iter().chain(self.max.iter()).all(|v|v.is_finite())&&self.min[0]<=self.max[0]&&self.min[1]<=self.max[1]&&self.min[2]<=self.max[2] }
    pub fn center(&self)->[f32;3]{[(self.min[0]+self.max[0])*0.5,(self.min[1]+self.max[1])*0.5,(self.min[2]+self.max[2])*0.5]}
    pub fn extents(&self)->[f32;3]{[(self.max[0]-self.min[0])*0.5,(self.max[1]-self.min[1])*0.5,(self.max[2]-self.min[2])*0.5]}
    pub fn translated(&self,t:[f32;3])->Option<Self>{if !t.iter().all(|v|v.is_finite()){return None}Some(Self{min:[self.min[0]+t[0],self.min[1]+t[1],self.min[2]+t[2]],max:[self.max[0]+t[0],self.max[1]+t[1],self.max[2]+t[2]]})}
    pub fn intersects_frustum(&self,frustum:&Frustum)->bool { if !self.validate(){return false} let c=self.center();let e=self.extents();frustum.planes.iter().all(|p|{let r=p.normal[0].abs()*e[0]+p.normal[1].abs()*e[1]+p.normal[2].abs()*e[2];p.signed_distance(c)>=-r}) }
    pub fn bounding_sphere(&self)->BoundingSphere{let c=self.center();let e=self.extents();BoundingSphere{center:c,radius:(e[0]*e[0]+e[1]*e[1]+e[2]*e[2]).sqrt()}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BvhLeaf { pub entity:u64, pub bounds:Aabb }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BvhNode { pub bounds:Aabb, pub start:usize, pub count:usize, pub left:Option<usize>, pub right:Option<usize> }
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bvh { pub nodes:Vec<BvhNode>, pub leaves:Vec<BvhLeaf> }
impl Bvh {
    pub fn build(mut leaves:Vec<BvhLeaf>)->Result<Self,BvhError>{
        if leaves.iter().any(|l|!l.bounds.validate()){return Err(BvhError::InvalidBounds)}
        leaves.sort_by_key(|l|l.entity);
        let mut b=Self{nodes:Vec::new(),leaves};
        if !b.leaves.is_empty(){b.build_node(0,b.leaves.len())?;}
        Ok(b)
    }
    fn build_node(&mut self,start:usize,end:usize)->Result<usize,BvhError>{
        let bounds=union_bounds(self.leaves[start..end].iter().map(|l|l.bounds)).ok_or(BvhError::InvalidBounds)?;
        let node=self.nodes.len();self.nodes.push(BvhNode{bounds,start,count:end-start,left:None,right:None});
        if end-start>2{
            let e=bounds.extents();let axis=if e[0]>=e[1]&&e[0]>=e[2]{0}else if e[1]>=e[2]{1}else{2};
            self.leaves[start..end].sort_by(|a,b|center_axis(a.bounds,axis).total_cmp(&center_axis(b.bounds,axis)).then_with(||a.entity.cmp(&b.entity)));
            let mid=start+(end-start)/2;
            let left=self.build_node(start,mid)?;let right=self.build_node(mid,end)?;
            self.nodes[node].left=Some(left);self.nodes[node].right=Some(right);
        }
        Ok(node)
    }
    pub fn query_frustum(&self,frustum:&Frustum)->Vec<u64>{
        let mut out=Vec::new();if self.nodes.is_empty(){return out}let mut stack=vec![0];
        while let Some(i)=stack.pop(){let n=&self.nodes[i];if !n.bounds.intersects_frustum(frustum){continue}if let Some(l)=n.left{stack.push(l);stack.push(n.right.unwrap());}else{for leaf in &self.leaves[n.start..n.start+n.count]{if leaf.bounds.intersects_frustum(frustum){out.push(leaf.entity)}}}}
        out.sort_unstable();out
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BvhError{InvalidBounds}
fn center_axis(b:Aabb,axis:usize)->f32{b.center()[axis]}
fn union_bounds<I:Iterator<Item=Aabb>>(mut it:I)->Option<Aabb>{let first=it.next()?;Some(it.fold(first,|a,b|Aabb{min:[a.min[0].min(b.min[0]),a.min[1].min(b.min[1]),a.min[2].min(b.min[2])],max:[a.max[0].max(b.max[0]),a.max[1].max(b.max[1]),a.max[2].max(b.max[2])]}))}

#[cfg(test)]
mod tests{use super::*;use crate::camera::CameraProjection;use crate::frustum::Frustum;fn f()->Frustum{Frustum::from_camera(CameraProjection::default()).unwrap()}fn l(id:u64,z:f32)->BvhLeaf{BvhLeaf{entity:id,bounds:Aabb{min:[-0.5,-0.5,z-0.5],max:[0.5,0.5,z+0.5]}}}
#[test]fn sphere_inside(){assert!(BoundingSphere{center:[0.0,0.0,-2.0],radius:0.5}.intersects_frustum(&f()));}
#[test]fn sphere_outside_far_plane(){assert!(!BoundingSphere{center:[0.0,0.0,-1001.0],radius:0.1}.intersects_frustum(&f()));}
#[test]fn aabb_inside(){assert!(Aabb{min:[-0.5,-0.5,-2.5],max:[0.5,0.5,-1.5]}.intersects_frustum(&f()));}
#[test]fn aabb_outside_side_plane(){assert!(!Aabb{min:[100.0,-1.0,-3.0],max:[101.0,1.0,-2.0]}.intersects_frustum(&f()));}
#[test]fn invalid_aabb_is_rejected(){assert!(!Aabb{min:[1.0,0.0,0.0],max:[-1.0,0.0,0.0]}.intersects_frustum(&f()));}
#[test]fn aabb_sphere_is_deterministic(){let s=Aabb{min:[-1.0,-2.0,-3.0],max:[3.0,2.0,1.0]}.bounding_sphere();assert_eq!(s.center,[1.0,0.0,-1.0]);assert_eq!(s.radius,12.0_f32.sqrt());}
#[test]fn translated_sphere_preserves_radius(){let s=BoundingSphere{center:[1.0,2.0,3.0],radius:4.0}.translated([-1.0,2.0,0.5]).unwrap();assert_eq!(s.center,[0.0,4.0,3.5]);assert_eq!(s.radius,4.0);}
#[test]fn translated_aabb_preserves_extent(){let a=Aabb{min:[-1.0,-2.0,-3.0],max:[1.0,2.0,3.0]}.translated([4.0,5.0,6.0]).unwrap();assert_eq!(a.min,[3.0,3.0,3.0]);assert_eq!(a.max,[5.0,7.0,9.0]);}
#[test]fn bvh_queries_deterministically(){let b=Bvh::build(vec![l(3,-3.0),l(1,-2.0),l(2,-1001.0)]).unwrap();assert_eq!(b.query_frustum(&f()),vec![1,3]);}
#[test]fn bvh_rejects_invalid_bounds(){let bad=BvhLeaf{entity:1,bounds:Aabb{min:[1.0,0.0,0.0],max:[-1.0,0.0,0.0]}};assert_eq!(Bvh::build(vec![bad]),Err(BvhError::InvalidBounds));}}

use super::frustum::Frustum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingSphere { pub center: [f32; 3], pub radius: f32 }

impl BoundingSphere {
    pub fn validate(&self) -> bool { self.center.iter().all(|v| v.is_finite()) && self.radius.is_finite() && self.radius >= 0.0 }
    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool { self.validate() && frustum.planes.iter().all(|plane| plane.signed_distance(self.center) >= -self.radius) }
    pub fn translated(&self, translation: [f32; 3]) -> Option<Self> { if !translation.iter().all(|v| v.is_finite()) { return None; } Some(Self { center: [self.center[0]+translation[0],self.center[1]+translation[1],self.center[2]+translation[2]], radius:self.radius }) }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min: [f32; 3], pub max: [f32; 3] }

impl Aabb {
    pub fn validate(&self) -> bool { self.min.iter().chain(self.max.iter()).all(|v| v.is_finite()) && self.min[0] <= self.max[0] && self.min[1] <= self.max[1] && self.min[2] <= self.max[2] }
    pub fn center(&self) -> [f32; 3] { [(self.min[0]+self.max[0])*0.5,(self.min[1]+self.max[1])*0.5,(self.min[2]+self.max[2])*0.5] }
    pub fn extents(&self) -> [f32; 3] { [(self.max[0]-self.min[0])*0.5,(self.max[1]-self.min[1])*0.5,(self.max[2]-self.min[2])*0.5] }
    pub fn translated(&self, translation: [f32; 3]) -> Option<Self> { if !translation.iter().all(|v| v.is_finite()) { return None; } Some(Self { min:[self.min[0]+translation[0],self.min[1]+translation[1],self.min[2]+translation[2]], max:[self.max[0]+translation[0],self.max[1]+translation[1],self.max[2]+translation[2]] }) }
    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool {
        if !self.validate() { return false; }
        let center=self.center(); let extents=self.extents();
        frustum.planes.iter().all(|plane| { let radius=plane.normal[0].abs()*extents[0]+plane.normal[1].abs()*extents[1]+plane.normal[2].abs()*extents[2]; plane.signed_distance(center)>=-radius })
    }
    pub fn bounding_sphere(&self) -> BoundingSphere { let c=self.center(); let e=self.extents(); BoundingSphere { center:c, radius:(e[0]*e[0]+e[1]*e[1]+e[2]*e[2]).sqrt() } }
}

#[cfg(test)]
mod tests {
    use super::*; use crate::camera::CameraProjection; use crate::frustum::Frustum;
    fn frustum()->Frustum { Frustum::from_camera(CameraProjection::default()).unwrap() }
    #[test] fn sphere_inside(){assert!(BoundingSphere{center:[0.0,0.0,-2.0],radius:0.5}.intersects_frustum(&frustum()));}
    #[test] fn sphere_outside_far_plane(){assert!(!BoundingSphere{center:[0.0,0.0,-1001.0],radius:0.1}.intersects_frustum(&frustum()));}
    #[test] fn aabb_inside(){assert!(Aabb{min:[-0.5,-0.5,-2.5],max:[0.5,0.5,-1.5]}.intersects_frustum(&frustum()));}
    #[test] fn aabb_outside_side_plane(){assert!(!Aabb{min:[100.0,-1.0,-3.0],max:[101.0,1.0,-2.0]}.intersects_frustum(&frustum()));}
    #[test] fn invalid_aabb_is_rejected(){assert!(!Aabb{min:[1.0,0.0,0.0],max:[-1.0,0.0,0.0]}.intersects_frustum(&frustum()));}
    #[test] fn aabb_sphere_is_deterministic(){let s=Aabb{min:[-1.0,-2.0,-3.0],max:[3.0,2.0,1.0]}.bounding_sphere();assert_eq!(s.center,[1.0,0.0,-1.0]);assert_eq!(s.radius,12.0_f32.sqrt());}
    #[test] fn translated_sphere_preserves_radius(){let s=BoundingSphere{center:[1.0,2.0,3.0],radius:4.0}.translated([-1.0,2.0,0.5]).unwrap();assert_eq!(s.center,[0.0,4.0,3.5]);assert_eq!(s.radius,4.0);}
    #[test] fn translated_aabb_preserves_extent(){let a=Aabb{min:[-1.0,-2.0,-3.0],max:[1.0,2.0,3.0]}.translated([4.0,5.0,6.0]).unwrap();assert_eq!(a.min,[3.0,3.0,3.0]);assert_eq!(a.max,[5.0,7.0,9.0]);}
}

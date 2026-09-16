use super::frustum::Frustum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingSphere { pub center: [f32; 3], pub radius: f32 }

impl BoundingSphere {
    pub fn validate(&self) -> bool { self.center.iter().all(|v| v.is_finite()) && self.radius.is_finite() && self.radius >= 0.0 }

    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool {
        self.validate() && frustum.planes.iter().all(|plane| plane.signed_distance(self.center) >= -self.radius)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min: [f32; 3], pub max: [f32; 3] }

impl Aabb {
    pub fn validate(&self) -> bool {
        self.min.iter().chain(self.max.iter()).all(|v| v.is_finite()) &&
            self.min[0] <= self.max[0] && self.min[1] <= self.max[1] && self.min[2] <= self.max[2]
    }

    pub fn center(&self) -> [f32; 3] { [(self.min[0]+self.max[0])*0.5,(self.min[1]+self.max[1])*0.5,(self.min[2]+self.max[2])*0.5] }
    pub fn extents(&self) -> [f32; 3] { [(self.max[0]-self.min[0])*0.5,(self.max[1]-self.min[1])*0.5,(self.max[2]-self.min[2])*0.5] }

    pub fn intersects_frustum(&self, frustum: &Frustum) -> bool {
        if !self.validate() { return false; }
        let center = self.center();
        let extents = self.extents();
        frustum.planes.iter().all(|plane| {
            let radius = plane.normal[0].abs()*extents[0] + plane.normal[1].abs()*extents[1] + plane.normal[2].abs()*extents[2];
            plane.signed_distance(center) >= -radius
        })
    }

    pub fn bounding_sphere(&self) -> BoundingSphere {
        let c=self.center(); let e=self.extents();
        BoundingSphere { center:c, radius:(e[0]*e[0]+e[1]*e[1]+e[2]*e[2]).sqrt() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::CameraProjection;
    use crate::frustum::Frustum;

    fn frustum() -> Frustum { Frustum::from_camera(CameraProjection::default()).unwrap() }

    #[test] fn sphere_inside() { let s=BoundingSphere{center:[0.0,0.0,-2.0],radius:0.5}; assert!(s.intersects_frustum(&frustum())); }
    #[test] fn sphere_outside_far_plane() { let s=BoundingSphere{center:[0.0,0.0,-1001.0],radius:0.1}; assert!(!s.intersects_frustum(&frustum())); }
    #[test] fn aabb_inside() { let a=Aabb{min:[-0.5,-0.5,-2.5],max:[0.5,0.5,-1.5]}; assert!(a.intersects_frustum(&frustum())); }
    #[test] fn aabb_outside_side_plane() { let a=Aabb{min:[100.0,-1.0,-3.0],max:[101.0,1.0,-2.0]}; assert!(!a.intersects_frustum(&frustum())); }
    #[test] fn invalid_aabb_is_rejected() { let a=Aabb{min:[1.0,0.0,0.0],max:[-1.0,0.0,0.0]}; assert!(!a.intersects_frustum(&frustum())); }
    #[test] fn aabb_sphere_is_deterministic() { let a=Aabb{min:[-1.0,-2.0,-3.0],max:[3.0,2.0,1.0]}; let s=a.bounding_sphere(); assert_eq!(s.center,[1.0,0.0,-1.0]); assert_eq!(s.radius,12.0_f32.sqrt()); }
}

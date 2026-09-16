use super::camera::CameraProjection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane { pub normal: [f32; 3], pub distance: f32 }

impl Plane {
    pub fn signed_distance(&self, point: [f32; 3]) -> f32 {
        self.normal[0] * point[0] + self.normal[1] * point[1] + self.normal[2] * point[2] + self.distance
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frustum { pub planes: [Plane; 6] }

impl Frustum {
    pub fn from_camera(projection: CameraProjection) -> Result<Self, FrustumError> {
        projection.validate().map_err(|_| FrustumError::InvalidProjection)?;
        let half_v = projection.fov_y_radians * 0.5;
        let half_h = (half_v.tan() * projection.aspect_ratio).atan();
        let near = projection.near;
        let far = projection.far;
        Ok(Self {
            planes: [
                Plane { normal: [0.0, 0.0, -1.0], distance: -near },
                Plane { normal: [0.0, 0.0, 1.0], distance: far },
                Plane { normal: [half_h.cos(), 0.0, -half_h.sin()], distance: 0.0 },
                Plane { normal: [-half_h.cos(), 0.0, -half_h.sin()], distance: 0.0 },
                Plane { normal: [0.0, -half_v.cos(), -half_v.sin()], distance: 0.0 },
                Plane { normal: [0.0, half_v.cos(), -half_v.sin()], distance: 0.0 },
            ],
        })
    }

    pub fn contains_point(&self, point: [f32; 3]) -> bool {
        self.planes.iter().all(|plane| plane.signed_distance(point) >= 0.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrustumError { InvalidProjection }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn origin_is_outside_due_to_near_plane() { let f=Frustum::from_camera(CameraProjection::default()).unwrap(); assert!(!f.contains_point([0.0,0.0,0.0])); }
    #[test] fn forward_point_is_inside() { let f=Frustum::from_camera(CameraProjection::default()).unwrap(); assert!(f.contains_point([0.0,0.0,-1.0])); }
    #[test] fn far_point_is_outside() { let f=Frustum::from_camera(CameraProjection::default()).unwrap(); assert!(!f.contains_point([0.0,0.0,-1001.0])); }
}

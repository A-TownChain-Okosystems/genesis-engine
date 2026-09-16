#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraProjection {
    pub fov_y_radians: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for CameraProjection {
    fn default() -> Self { Self { fov_y_radians: core::f32::consts::FRAC_PI_3, aspect_ratio: 16.0 / 9.0, near: 0.1, far: 1000.0 } }
}

impl CameraProjection {
    pub fn validate(&self) -> Result<(), CameraError> {
        if !self.fov_y_radians.is_finite() || !self.aspect_ratio.is_finite() || !self.near.is_finite() || !self.far.is_finite() { return Err(CameraError::NonFinite); }
        if !(0.0 < self.fov_y_radians && self.fov_y_radians < core::f32::consts::PI) { return Err(CameraError::InvalidFov); }
        if self.aspect_ratio <= 0.0 { return Err(CameraError::InvalidAspectRatio); }
        if !(self.near > 0.0 && self.far > self.near) { return Err(CameraError::InvalidClipRange); }
        Ok(())
    }

    pub fn perspective_matrix(&self) -> Result<[[f32; 4]; 4], CameraError> {
        self.validate()?;
        let f = 1.0 / (self.fov_y_radians * 0.5).tan();
        let nf = 1.0 / (self.near - self.far);
        Ok([[f / self.aspect_ratio, 0.0, 0.0, 0.0],[0.0, f, 0.0, 0.0],[0.0, 0.0, (self.far + self.near) * nf, -1.0],[0.0, 0.0, (2.0 * self.far * self.near) * nf, 0.0]])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraView {
    pub position: [f32; 3],
    pub forward: [f32; 3],
    pub up: [f32; 3],
}

impl Default for CameraView { fn default() -> Self { Self { position: [0.0,0.0,0.0], forward: [0.0,0.0,-1.0], up: [0.0,1.0,0.0] } } }

impl CameraView {
    pub fn validate(&self) -> Result<(), CameraError> {
        if !self.position.iter().chain(self.forward.iter()).chain(self.up.iter()).all(|v| v.is_finite()) { return Err(CameraError::NonFinite); }
        if length(self.forward) <= f32::EPSILON || length(self.up) <= f32::EPSILON { return Err(CameraError::InvalidBasis); }
        if length(cross(self.forward, self.up)) <= f32::EPSILON { return Err(CameraError::InvalidBasis); }
        Ok(())
    }

    pub fn view_matrix(&self) -> Result<[[f32; 4]; 4], CameraError> {
        self.validate()?;
        let forward = normalize(self.forward);
        let right = normalize(cross(forward, self.up));
        let up = cross(right, forward);
        Ok([[right[0], up[0], -forward[0], 0.0],[right[1], up[1], -forward[1], 0.0],[right[2], up[2], -forward[2], 0.0],[-dot(right,self.position), -dot(up,self.position), dot(forward,self.position), 1.0]])
    }
}

fn dot(a:[f32;3],b:[f32;3])->f32 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn cross(a:[f32;3],b:[f32;3])->[f32;3] {[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
fn length(v:[f32;3])->f32 { dot(v,v).sqrt() }
fn normalize(v:[f32;3])->[f32;3] { let n=length(v); [v[0]/n,v[1]/n,v[2]/n] }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraError { NonFinite, InvalidFov, InvalidAspectRatio, InvalidClipRange, InvalidBasis }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn default_projection_is_valid() { assert!(CameraProjection::default().validate().is_ok()); }
    #[test] fn invalid_clip_range_is_rejected() { let p=CameraProjection { near:10.0, far:1.0, ..Default::default() }; assert_eq!(p.validate(), Err(CameraError::InvalidClipRange)); }
    #[test] fn projection_matrix_has_expected_signs() { let m=CameraProjection::default().perspective_matrix().unwrap(); assert!(m[0][0]>0.0 && m[1][1]>0.0 && m[2][3]<0.0); }
    #[test] fn default_view_is_valid() { assert!(CameraView::default().validate().is_ok()); }
    #[test] fn view_matrix_keeps_origin_at_origin() { let m=CameraView::default().view_matrix().unwrap(); assert_eq!(m[3], [0.0,0.0,0.0,1.0]); }
    #[test] fn collinear_basis_is_rejected() { let v=CameraView { up:[0.0,0.0,-1.0], ..Default::default() }; assert_eq!(v.validate(),Err(CameraError::InvalidBasis)); }
}

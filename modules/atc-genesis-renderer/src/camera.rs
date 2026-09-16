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
        Ok([
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (self.far + self.near) * nf, -1.0],
            [0.0, 0.0, (2.0 * self.far * self.near) * nf, 0.0],
        ])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraError { NonFinite, InvalidFov, InvalidAspectRatio, InvalidClipRange }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn default_projection_is_valid() { assert!(CameraProjection::default().validate().is_ok()); }
    #[test] fn invalid_clip_range_is_rejected() { let p=CameraProjection { near:10.0, far:1.0, ..Default::default() }; assert_eq!(p.validate(), Err(CameraError::InvalidClipRange)); }
    #[test] fn projection_matrix_has_expected_signs() { let m=CameraProjection::default().perspective_matrix().unwrap(); assert!(m[0][0]>0.0 && m[1][1]>0.0 && m[2][3]<0.0); }
}

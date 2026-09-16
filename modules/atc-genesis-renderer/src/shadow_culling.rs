//! Deterministic shadow-caster selection for backend-neutral shadow passes.
//!
//! Culling is conservative: a caster is retained when its bounding sphere
//! intersects the cascade volume. Backend-specific depth rendering remains
//! outside this module.

use crate::bounds::BoundingSphere;
use crate::camera::{CameraProjection, CameraView};
use crate::shadow_math::directional_shadow_matrix;
use crate::shadows::{ShadowCascade, ShadowMapResolution};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowCasterBounds {
    pub entity_id: u64,
    pub sphere: BoundingSphere,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShadowCasterSet {
    pub entity_ids: Vec<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowCullingError {
    InvalidCaster,
    InvalidCamera,
    InvalidCascade,
    InvalidResolution,
}

/// Selects casters conservatively for a directional-light cascade.
///
/// The cascade is expanded by each caster radius in camera space. This avoids
/// false negatives at cascade boundaries while remaining deterministic.
pub fn cull_directional_casters(
    view: &CameraView,
    projection: &CameraProjection,
    light_direction: [f32; 3],
    cascade: ShadowCascade,
    casters: &[ShadowCasterBounds],
) -> Result<ShadowCasterSet, ShadowCullingError> {
    view.validate().map_err(|_| ShadowCullingError::InvalidCamera)?;
    projection.validate().map_err(|_| ShadowCullingError::InvalidCamera)?;
    if !cascade.near.is_finite() || !cascade.far.is_finite()
        || cascade.near <= 0.0 || cascade.far <= cascade.near
        || cascade.far > projection.far
    {
        return Err(ShadowCullingError::InvalidCascade);
    }

    let _projection = directional_shadow_matrix(
        view,
        projection,
        light_direction,
        cascade.near,
        cascade.far,
    ).map_err(|_| ShadowCullingError::InvalidCascade)?;

    let forward = normalize(view.forward).ok_or(ShadowCullingError::InvalidCamera)?;
    let right = normalize(cross(forward, view.up)).ok_or(ShadowCullingError::InvalidCamera)?;
    let up = normalize(cross(right, forward)).ok_or(ShadowCullingError::InvalidCamera)?;
    let tan_half = (projection.fov_y_radians * 0.5).tan();
    let center_distance = (cascade.near + cascade.far) * 0.5;
    let center = add(view.position, scale(forward, center_distance));
    let half_y = cascade.far * tan_half;
    let half_x = half_y * projection.aspect_ratio;

    let mut ids = Vec::new();
    for caster in casters {
        if !caster.sphere.validate() || !caster.sphere.radius.is_finite() {
            return Err(ShadowCullingError::InvalidCaster);
        }
        let delta = sub(caster.sphere.center, view.position);
        let depth = dot(delta, forward);
        let radius = caster.sphere.radius;
        if depth + radius < cascade.near || depth - radius > cascade.far {
            continue;
        }
        let x = dot(delta, right);
        let y = dot(delta, up);
        let depth_for_extent = depth.max(cascade.near);
        let extent_y = depth_for_extent * tan_half;
        let extent_x = extent_y * projection.aspect_ratio;
        if x.abs() <= extent_x + radius && y.abs() <= extent_y + radius {
            ids.push(caster.entity_id);
        }
    }
    ids.sort_unstable();
    ids.dedup();
    let _ = (center, half_x, half_y);
    Ok(ShadowCasterSet { entity_ids: ids })
}

/// Validates that a configured shadow-map resolution is usable for culling
/// metadata. Kept separate so future GPU implementations can share the rule.
pub fn validate_shadow_resolution(resolution: ShadowMapResolution) -> Result<u32, ShadowCullingError> {
    let pixels = resolution.pixels();
    if pixels == 0 { return Err(ShadowCullingError::InvalidResolution); }
    Ok(pixels)
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn scale(a: [f32; 3], s: f32) -> [f32; 3] { [a[0] * s, a[1] * s, a[2] * s] }
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn normalize(v: [f32; 3]) -> Option<[f32; 3]> { let n = dot(v, v).sqrt(); if !n.is_finite() || n <= f32::EPSILON { None } else { Some([v[0] / n, v[1] / n, v[2] / n]) } }

#[cfg(test)]
mod tests {
    use super::*;

    fn caster(id: u64, z: f32) -> ShadowCasterBounds {
        ShadowCasterBounds { entity_id: id, sphere: BoundingSphere { center: [0.0, 0.0, z], radius: 0.5 } }
    }

    #[test]
    fn culling_is_deterministic_and_sorted() {
        let v = CameraView::default();
        let p = CameraProjection::default();
        let c = ShadowCascade { near: 0.1, far: 100.0 };
        let result = cull_directional_casters(&v, &p, [0.2, -1.0, 0.3], c, &[caster(8, -2.0), caster(3, -4.0)]).unwrap();
        assert_eq!(result.entity_ids, vec![3, 8]);
    }

    #[test]
    fn objects_outside_depth_are_rejected() {
        let v = CameraView::default();
        let p = CameraProjection::default();
        let c = ShadowCascade { near: 1.0, far: 10.0 };
        let result = cull_directional_casters(&v, &p, [0.0, -1.0, 0.0], c, &[caster(1, -50.0)]).unwrap();
        assert!(result.entity_ids.is_empty());
    }

    #[test]
    fn invalid_caster_is_fail_closed() {
        let v = CameraView::default();
        let p = CameraProjection::default();
        let c = ShadowCascade { near: 0.1, far: 10.0 };
        let bad = ShadowCasterBounds { entity_id: 1, sphere: BoundingSphere { center: [f32::NAN, 0.0, -2.0], radius: 1.0 } };
        assert_eq!(cull_directional_casters(&v, &p, [0.0, -1.0, 0.0], c, &[bad]), Err(ShadowCullingError::InvalidCaster));
    }

    #[test]
    fn resolution_is_validated() {
        assert_eq!(validate_shadow_resolution(ShadowMapResolution::R2048).unwrap(), 2048);
    }
}

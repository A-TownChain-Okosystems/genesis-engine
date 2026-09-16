//! Backend-neutral shadow projection math.
//!
//! This module produces deterministic light-space matrices for shadow planning.
//! GPU resource allocation and API-specific command encoding remain backend work.

use crate::camera::{CameraProjection, CameraView};
use crate::shadows::ShadowMapResolution;

pub type Mat4 = [[f32; 4]; 4];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowMatrix { pub matrix: Mat4 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowMathError { InvalidCamera, InvalidLightDirection, NonFinite }

pub fn directional_shadow_matrix(view: &CameraView, projection: &CameraProjection, light_direction: [f32; 3], near: f32, far: f32) -> Result<ShadowMatrix, ShadowMathError> {
    directional_shadow_matrix_stable(view, projection, light_direction, near, far, None)
}

/// Builds a directional-light shadow matrix with optional texel snapping.
///
/// Snapping is performed in light space using the shadow-map resolution. The
/// resulting translation is quantized to whole texels, reducing camera-motion
/// shimmer while keeping the projection deterministic. The un-stabilized path
/// remains available through `directional_shadow_matrix`.
pub fn directional_shadow_matrix_stable(
    view: &CameraView,
    projection: &CameraProjection,
    light_direction: [f32; 3],
    near: f32,
    far: f32,
    resolution: Option<ShadowMapResolution>,
) -> Result<ShadowMatrix, ShadowMathError> {
    view.validate().map_err(|_| ShadowMathError::InvalidCamera)?;
    projection.validate().map_err(|_| ShadowMathError::InvalidCamera)?;
    if !near.is_finite() || !far.is_finite() || !(near > 0.0 && far > near && far <= projection.far) { return Err(ShadowMathError::InvalidCamera); }
    let ld = normalize(light_direction).ok_or(ShadowMathError::InvalidLightDirection)?;
    let forward = normalize(view.forward).ok_or(ShadowMathError::InvalidCamera)?;
    let right = normalize(cross(forward, view.up)).ok_or(ShadowMathError::InvalidCamera)?;
    let up = normalize(cross(right, forward)).ok_or(ShadowMathError::InvalidCamera)?;
    let tan_half = (projection.fov_y_radians * 0.5).tan();
    let half_y = far * tan_half;
    let half_x = half_y * projection.aspect_ratio;
    let center_distance = (near + far) * 0.5;
    let center = add(view.position, scale(forward, center_distance));
    let corners = [
        add(add(center, scale(right, -half_x)), scale(up, -half_y)),
        add(add(center, scale(right,  half_x)), scale(up, -half_y)),
        add(add(center, scale(right, -half_x)), scale(up,  half_y)),
        add(add(center, scale(right,  half_x)), scale(up,  half_y)),
    ];
    let radius = corners.iter().map(|p| length(sub(*p, center))).fold(0.0, f32::max).max(1e-3);
    let light_up = choose_up(ld);
    let light_right = normalize(cross(light_up, ld)).ok_or(ShadowMathError::InvalidLightDirection)?;
    let light_up = normalize(cross(ld, light_right)).ok_or(ShadowMathError::InvalidLightDirection)?;
    let mut eye = sub(center, scale(ld, radius));
    let mut view_matrix = look_at(eye, center, light_up).ok_or(ShadowMathError::InvalidLightDirection)?;
    if let Some(map) = resolution {
        let texel = (2.0 * radius) / map.pixels() as f32;
        let center_ls = transform_point(view_matrix, center);
        let snapped_x = (center_ls[0] / texel).round() * texel;
        let snapped_y = (center_ls[1] / texel).round() * texel;
        let delta_x = snapped_x - center_ls[0];
        let delta_y = snapped_y - center_ls[1];
        eye = add(eye, add(scale(light_right, -delta_x), scale(light_up, -delta_y)));
        view_matrix = look_at(eye, center, light_up).ok_or(ShadowMathError::InvalidLightDirection)?;
    }
    let ortho = orthographic(-radius, radius, -radius, radius, 0.0, 2.0 * radius);
    let matrix = mul(ortho, view_matrix);
    if !matrix.iter().flatten().all(|v| v.is_finite()) { return Err(ShadowMathError::NonFinite); }
    Ok(ShadowMatrix { matrix })
}

fn choose_up(direction:[f32;3])->[f32;3] { if direction[1].abs() < 0.9 { [0.0,1.0,0.0] } else { [1.0,0.0,0.0] } }
fn add(a:[f32;3],b:[f32;3])->[f32;3]{[a[0]+b[0],a[1]+b[1],a[2]+b[2]]}
fn sub(a:[f32;3],b:[f32;3])->[f32;3]{[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
fn scale(a:[f32;3],s:f32)->[f32;3]{[a[0]*s,a[1]*s,a[2]*s]}
fn dot(a:[f32;3],b:[f32;3])->f32{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn cross(a:[f32;3],b:[f32;3])->[f32;3]{[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
fn length(v:[f32;3])->f32{dot(v,v).sqrt()}
fn normalize(v:[f32;3])->Option<[f32;3]>{let n=length(v);if !n.is_finite()||n<=f32::EPSILON{None}else{Some([v[0]/n,v[1]/n,v[2]/n])}}
fn look_at(eye:[f32;3],center:[f32;3],up:[f32;3])->Option<Mat4>{let f=normalize(sub(center,eye))?;let r=normalize(cross(f,up))?;let u=cross(r,f);Some([[r[0],u[0],-f[0],0.0],[r[1],u[1],-f[1],0.0],[r[2],u[2],-f[2],0.0],[-dot(r,eye),-dot(u,eye),dot(f,eye),1.0]])}
fn transform_point(m:Mat4,p:[f32;3])->[f32;3]{[p[0]*m[0][0]+p[1]*m[1][0]+p[2]*m[2][0]+m[3][0],p[0]*m[0][1]+p[1]*m[1][1]+p[2]*m[2][1]+m[3][1],p[0]*m[0][2]+p[1]*m[1][2]+p[2]*m[2][2]+m[3][2]]}
fn orthographic(l:f32,r:f32,b:f32,t:f32,n:f32,f:f32)->Mat4{[[2.0/(r-l),0.0,0.0,0.0],[0.0,2.0/(t-b),0.0,0.0],[0.0,0.0,-2.0/(f-n),0.0],[-(r+l)/(r-l),-(t+b)/(t-b),-(f+n)/(f-n),1.0]]}
fn mul(a:Mat4,b:Mat4)->Mat4{let mut o=[[0.0;4];4];for i in 0..4{for j in 0..4{for k in 0..4{o[i][j]+=a[i][k]*b[k][j];}}}o}

#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn deterministic_matrix(){let v=CameraView::default();let p=CameraProjection::default();let a=directional_shadow_matrix(&v,&p,[0.3,-1.0,0.2],0.1,100.0).unwrap();let b=directional_shadow_matrix(&v,&p,[0.3,-1.0,0.2],0.1,100.0).unwrap();assert_eq!(a,b);}
 #[test] fn zero_light_direction_rejected(){let r=directional_shadow_matrix(&CameraView::default(),&CameraProjection::default(),[0.0;3],0.1,100.0);assert_eq!(r,Err(ShadowMathError::InvalidLightDirection));}
 #[test] fn invalid_range_rejected(){let r=directional_shadow_matrix(&CameraView::default(),&CameraProjection::default(),[0.0,-1.0,0.0],10.0,1.0);assert_eq!(r,Err(ShadowMathError::InvalidCamera));}
 #[test] fn stabilized_matrix_is_deterministic(){let v=CameraView::default();let p=CameraProjection::default();let a=directional_shadow_matrix_stable(&v,&p,[0.3,-1.0,0.2],0.1,100.0,Some(ShadowMapResolution::R2048)).unwrap();let b=directional_shadow_matrix_stable(&v,&p,[0.3,-1.0,0.2],0.1,100.0,Some(ShadowMapResolution::R2048)).unwrap();assert_eq!(a,b);}
}
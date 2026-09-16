//! Deterministic shadow configuration and CPU-side shadow projection helpers.
//!
//! This module owns shadow policy and cascade selection. Backend-specific depth
//! textures and GPU commands remain in the graphics backend.

use crate::lighting::{DirectionalLight, LightId, LightKind, LightRegistry, PointLight, SpotLight};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowMode { None, Hard, Pcf }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowMapResolution { R256, R512, R1024, R2048, R4096 }

impl ShadowMapResolution {
    pub const fn pixels(self) -> u32 { match self { Self::R256=>256, Self::R512=>512, Self::R1024=>1024, Self::R2048=>2048, Self::R4096=>4096 } }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowConfig {
    pub mode: ShadowMode,
    pub resolution: ShadowMapResolution,
    pub bias: f32,
    pub normal_bias: f32,
    pub max_distance: f32,
    pub cascade_count: u8,
}

impl Default for ShadowConfig {
    fn default() -> Self { Self { mode: ShadowMode::Pcf, resolution: ShadowMapResolution::R2048, bias: 0.001, normal_bias: 0.01, max_distance: 2000.0, cascade_count: 4 } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowError { InvalidBias, InvalidDistance, InvalidCascadeCount, UnsupportedLight }

impl ShadowConfig {
    pub fn validate(&self) -> Result<(), ShadowError> {
        if !self.bias.is_finite() || self.bias < 0.0 || !self.normal_bias.is_finite() || self.normal_bias < 0.0 { return Err(ShadowError::InvalidBias); }
        if !self.max_distance.is_finite() || self.max_distance <= 0.0 { return Err(ShadowError::InvalidDistance); }
        if self.cascade_count == 0 || self.cascade_count > 8 { return Err(ShadowError::InvalidCascadeCount); }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowCaster { pub entity_id: u64, pub center: [f32; 3], pub radius: f32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowCascade { pub near: f32, pub far: f32 }

pub fn directional_cascades(camera_near: f32, camera_far: f32, config: &ShadowConfig) -> Result<Vec<ShadowCascade>, ShadowError> {
    config.validate()?;
    if !camera_near.is_finite() || !camera_far.is_finite() || camera_near < 0.0 || camera_far <= camera_near { return Err(ShadowError::InvalidDistance); }
    let far = camera_far.min(config.max_distance);
    let n = config.cascade_count as usize;
    let mut result = Vec::with_capacity(n);
    let ratio = far / camera_near.max(0.001);
    for i in 0..n {
        let a = i as f32 / n as f32;
        let b = (i + 1) as f32 / n as f32;
        let logarithmic_a = camera_near.max(0.001) * ratio.powf(a);
        let logarithmic_b = camera_near.max(0.001) * ratio.powf(b);
        let uniform_a = camera_near + (far - camera_near) * a;
        let uniform_b = camera_near + (far - camera_near) * b;
        let near = logarithmic_a * 0.75 + uniform_a * 0.25;
        let split_far = logarithmic_b * 0.75 + uniform_b * 0.25;
        result.push(ShadowCascade { near, far: split_far });
    }
    if let Some(last) = result.last_mut() { last.far = far; }
    Ok(result)
}

pub fn light_supports_shadows(light: &LightKind) -> bool {
    match light { LightKind::Directional(DirectionalLight{..}) | LightKind::Point(PointLight{..}) | LightKind::Spot(SpotLight{..}) => true }
}

pub fn shadow_lights(registry: &LightRegistry) -> Vec<LightId> {
    registry.iter().filter(|light| light_supports_shadows(&light.kind)).map(|light| light.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lighting::{Light, LightId, LightKind, PointLight};
    #[test] fn cascades_are_monotonic_and_end_at_limit() { let c=ShadowConfig::default(); let s=directional_cascades(0.1,5000.0,&c).unwrap(); assert_eq!(s.len(),4); assert!((s.last().unwrap().far-2000.0).abs()<0.001); for w in s.windows(2){assert!(w[0].far<=w[1].near);} }
    #[test] fn invalid_config_is_rejected() { let mut c=ShadowConfig::default(); c.cascade_count=0; assert_eq!(c.validate(),Err(ShadowError::InvalidCascadeCount)); }
    #[test] fn shadow_lights_are_deterministic() { let mut r=LightRegistry::default(); r.insert(Light{id:LightId(2),kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0})}).unwrap(); r.insert(Light{id:LightId(1),kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0})}).unwrap(); assert_eq!(shadow_lights(&r),vec![LightId(1),LightId(2)]); }
}

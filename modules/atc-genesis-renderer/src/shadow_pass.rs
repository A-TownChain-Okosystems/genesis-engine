//! Backend-neutral shadow pass planning.
//!
//! This module turns validated shadow policy into deterministic pass metadata.
//! It does not allocate GPU resources or issue API-specific commands.

use crate::lighting::{LightId, LightKind, LightRegistry};
use crate::shadows::{directional_cascades, ShadowConfig, ShadowError, ShadowMapResolution};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowProjection { DirectionalCascade, PointCube, SpotPerspective }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowPassId { pub light: LightId, pub slice: u8 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowPassDesc {
    pub id: ShadowPassId,
    pub projection: ShadowProjection,
    pub resolution: ShadowMapResolution,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShadowFramePlan {
    pub passes: Vec<ShadowPassDesc>,
}

impl ShadowFramePlan {
    pub fn empty() -> Self { Self { passes: Vec::new() } }

    pub fn build(registry: &LightRegistry, camera_near: f32, camera_far: f32, config: &ShadowConfig) -> Result<Self, ShadowError> {
        config.validate()?;
        let mut passes = Vec::new();
        for light in registry.iter() {
            match light.kind {
                LightKind::Directional(_) => {
                    let cascades = directional_cascades(camera_near, camera_far, config)?;
                    for slice in 0..cascades.len() as u8 {
                        passes.push(ShadowPassDesc { id: ShadowPassId { light: light.id, slice }, projection: ShadowProjection::DirectionalCascade, resolution: config.resolution });
                    }
                }
                LightKind::Point(_) => passes.push(ShadowPassDesc { id: ShadowPassId { light: light.id, slice: 0 }, projection: ShadowProjection::PointCube, resolution: config.resolution }),
                LightKind::Spot(_) => passes.push(ShadowPassDesc { id: ShadowPassId { light: light.id, slice: 0 }, projection: ShadowProjection::SpotPerspective, resolution: config.resolution }),
            }
        }
        Ok(Self { passes })
    }

    pub fn sort_deterministic(&mut self) {
        self.passes.sort_by_key(|pass| (pass.id.light, pass.id.slice));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lighting::{DirectionalLight, Light, LightId};
    #[test]
    fn directional_light_expands_to_cascades() {
        let mut r=LightRegistry::default();
        r.insert(Light { id:LightId(7), kind:LightKind::Directional(DirectionalLight{direction:[0.0,-1.0,0.0],color:[1.0;3],intensity:1.0}) }).unwrap();
        let plan=ShadowFramePlan::build(&r,0.1,1000.0,&ShadowConfig::default()).unwrap();
        assert_eq!(plan.passes.len(),4);
        assert!(plan.passes.iter().all(|p|p.projection==ShadowProjection::DirectionalCascade));
    }
    #[test]
    fn plan_order_is_stable() {
        let mut r=LightRegistry::default();
        r.insert(Light { id:LightId(2), kind:LightKind::Point(crate::lighting::PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0}) }).unwrap();
        r.insert(Light { id:LightId(1), kind:LightKind::Point(crate::lighting::PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0}) }).unwrap();
        let plan=ShadowFramePlan::build(&r,0.1,100.0,&ShadowConfig{cascade_count:1,..Default::default()}).unwrap();
        assert_eq!(plan.passes[0].id.light,LightId(1));
        assert_eq!(plan.passes[1].id.light,LightId(2));
    }
}

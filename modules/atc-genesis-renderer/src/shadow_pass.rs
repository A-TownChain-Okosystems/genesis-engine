//! Backend-neutral shadow pass planning and render-graph integration.

use crate::lighting::{LightId, LightKind, LightRegistry};
use crate::render_graph::{RenderGraphError, RenderPassDesc, RenderPassGraph, RenderPassId, RenderResourceId, ResourceAccess, ResourceUse};
use crate::shadows::{directional_cascades, ShadowConfig, ShadowError, ShadowMapResolution};

const SHADOW_PASS_NAMESPACE: u128 = 0x5348_4144_4f57_5041_5353_0000_0000_0000_0000;
const SHADOW_RESOURCE_NAMESPACE: u128 = 0x5348_4144_4f57_5245_534f_5552_4345_0000_0000;
const LIGHTING_PASS_ID: RenderPassId = RenderPassId(0x4c49_4748_5449_4e47_0000_0000_0000_0001);
const LIGHTING_RESOURCE_ID: RenderResourceId = RenderResourceId(0x4c49_4748_5449_4e47_5f52_4553_4f55_5243_45);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowProjection { DirectionalCascade, PointCube, SpotPerspective }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowPassId { pub light: LightId, pub slice: u8 }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowPassDesc { pub id: ShadowPassId, pub projection: ShadowProjection, pub resolution: ShadowMapResolution }
#[derive(Clone, Debug, PartialEq)]
pub struct ShadowFramePlan { pub passes: Vec<ShadowPassDesc> }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowGraphError { LightIdTooLarge, Graph(RenderGraphError) }
impl From<RenderGraphError> for ShadowGraphError { fn from(value: RenderGraphError) -> Self { Self::Graph(value) } }

fn pass_id(id: ShadowPassId) -> Result<RenderPassId, ShadowGraphError> {
    if id.light.0 > (u128::MAX >> 8) { return Err(ShadowGraphError::LightIdTooLarge); }
    Ok(RenderPassId(SHADOW_PASS_NAMESPACE | (id.light.0 << 8) | id.slice as u128))
}
fn resource_id(id: ShadowPassId) -> Result<RenderResourceId, ShadowGraphError> {
    if id.light.0 > (u128::MAX >> 8) { return Err(ShadowGraphError::LightIdTooLarge); }
    Ok(RenderResourceId(SHADOW_RESOURCE_NAMESPACE | (id.light.0 << 8) | id.slice as u128))
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
                    for slice in 0..cascades.len() as u8 { passes.push(ShadowPassDesc { id: ShadowPassId { light: light.id, slice }, projection: ShadowProjection::DirectionalCascade, resolution: config.resolution }); }
                }
                LightKind::Point(_) => passes.push(ShadowPassDesc { id: ShadowPassId { light: light.id, slice: 0 }, projection: ShadowProjection::PointCube, resolution: config.resolution }),
                LightKind::Spot(_) => passes.push(ShadowPassDesc { id: ShadowPassId { light: light.id, slice: 0 }, projection: ShadowProjection::SpotPerspective, resolution: config.resolution }),
            }
        }
        Ok(Self { passes })
    }
    pub fn sort_deterministic(&mut self) { self.passes.sort_by_key(|pass| (pass.id.light, pass.id.slice)); }
    /// Adds shadow write passes and a lighting read pass, enforcing Shadow -> Lighting ordering.
    pub fn populate_render_graph(&self, graph: &mut RenderPassGraph) -> Result<(), ShadowGraphError> {
        let mut dependencies = Vec::with_capacity(self.passes.len());
        for pass in &self.passes {
            let id = pass_id(pass.id)?;
            let resource = resource_id(pass.id)?;
            graph.add_pass(RenderPassDesc { id, resources: vec![ResourceUse { resource, access: ResourceAccess::Write }], depends_on: Vec::new() })?;
            dependencies.push(id);
        }
        let mut resources = Vec::with_capacity(self.passes.len() + 1);
        resources.push(ResourceUse { resource: LIGHTING_RESOURCE_ID, access: ResourceAccess::ReadWrite });
        for pass in &self.passes { resources.push(ResourceUse { resource: resource_id(pass.id)?, access: ResourceAccess::Read }); }
        graph.add_pass(RenderPassDesc { id: LIGHTING_PASS_ID, resources, depends_on: dependencies })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lighting::{DirectionalLight, Light, LightId, PointLight};
    #[test] fn directional_light_expands_to_cascades() { let mut r=LightRegistry::default(); r.insert(Light{id:LightId(7),kind:LightKind::Directional(DirectionalLight{direction:[0.0,-1.0,0.0],color:[1.0;3],intensity:1.0})}).unwrap(); let plan=ShadowFramePlan::build(&r,0.1,1000.0,&ShadowConfig::default()).unwrap(); assert_eq!(plan.passes.len(),4); }
    #[test] fn plan_order_is_stable() { let mut r=LightRegistry::default(); r.insert(Light{id:LightId(2),kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0})}).unwrap(); r.insert(Light{id:LightId(1),kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0})}).unwrap(); let plan=ShadowFramePlan::build(&r,0.1,100.0,&ShadowConfig{cascade_count:1,..Default::default()}).unwrap(); assert_eq!(plan.passes[0].id.light,LightId(1)); }
    #[test] fn graph_enforces_shadow_before_lighting() { let mut r=LightRegistry::default(); r.insert(Light{id:LightId(3),kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0})}).unwrap(); let plan=ShadowFramePlan::build(&r,0.1,100.0,&ShadowConfig{cascade_count:1,..Default::default()}).unwrap(); let mut graph=RenderPassGraph::default(); plan.populate_render_graph(&mut graph).unwrap(); let ordered=graph.ordered_passes().unwrap(); assert_eq!(ordered.last(),Some(&LIGHTING_PASS_ID)); assert_eq!(ordered.len(),2); }
    #[test] fn graph_rejects_unrepresentable_light_id() { let mut r=LightRegistry::default(); r.insert(Light{id:LightId(u128::MAX),kind:LightKind::Point(PointLight{position:[0.0;3],color:[1.0;3],intensity:1.0,range:10.0})}).unwrap(); let plan=ShadowFramePlan::build(&r,0.1,100.0,&ShadowConfig::default()).unwrap(); let mut graph=RenderPassGraph::default(); assert_eq!(plan.populate_render_graph(&mut graph),Err(ShadowGraphError::LightIdTooLarge)); }
}

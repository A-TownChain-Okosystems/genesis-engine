use super::{MaterialId, MeshHandle, RenderCommand, RenderCommandError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShaderId(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PipelineId(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderStage { Vertex, Fragment, Compute }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveTopology { TriangleList, TriangleStrip, LineList, LineStrip, PointList }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShaderModule { pub id: ShaderId, pub stage: ShaderStage }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GraphicsPipelineDesc {
    pub id: PipelineId,
    pub vertex: ShaderId,
    pub fragment: ShaderId,
    pub topology: PrimitiveTopology,
}

#[derive(Default)]
pub struct RenderPipelineRegistry { pipelines: Vec<GraphicsPipelineDesc> }

impl RenderPipelineRegistry {
    pub fn register(&mut self, pipeline: GraphicsPipelineDesc) -> Result<(), PipelineError> {
        if self.pipelines.iter().any(|p| p.id == pipeline.id) { return Err(PipelineError::DuplicateId); }
        if pipeline.vertex == pipeline.fragment { return Err(PipelineError::InvalidShaderPair); }
        self.pipelines.push(pipeline);
        self.pipelines.sort_by_key(|p| p.id);
        Ok(())
    }
    pub fn get(&self, id: PipelineId) -> Option<&GraphicsPipelineDesc> { self.pipelines.iter().find(|p| p.id == id) }
    pub fn len(&self) -> usize { self.pipelines.len() }
    pub fn is_empty(&self) -> bool { self.pipelines.is_empty() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PipelineError { DuplicateId, InvalidShaderPair }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawPacket { pub mesh: MeshHandle, pub material: MaterialId, pub pipeline: PipelineId }

impl DrawPacket {
    pub fn validate(&self, registry: &RenderPipelineRegistry) -> Result<(), PipelineError> {
        if registry.get(self.pipeline).is_some() { Ok(()) } else { Err(PipelineError::InvalidShaderPair) }
    }
}

pub fn command_kind(command: &RenderCommand) -> &'static str {
    match command { RenderCommand::BeginFrame(_) => "begin_frame", RenderCommand::SetCamera { .. } => "set_camera", RenderCommand::Draw { .. } => "draw", RenderCommand::EndFrame => "end_frame" }
}

pub fn command_error_is_recoverable(error: RenderCommandError) -> bool {
    matches!(error, RenderCommandError::FrameNotOpen | RenderCommandError::FrameAlreadyOpen)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn registry_is_deterministic() {
        let mut r=RenderPipelineRegistry::default();
        r.register(GraphicsPipelineDesc{id:PipelineId(2),vertex:ShaderId(2),fragment:ShaderId(3),topology:PrimitiveTopology::TriangleList}).unwrap();
        r.register(GraphicsPipelineDesc{id:PipelineId(1),vertex:ShaderId(4),fragment:ShaderId(5),topology:PrimitiveTopology::TriangleList}).unwrap();
        assert_eq!(r.get(PipelineId(1)).unwrap().vertex,ShaderId(4));
    }
    #[test] fn duplicate_ids_rejected() {
        let mut r=RenderPipelineRegistry::default();
        let p=GraphicsPipelineDesc{id:PipelineId(1),vertex:ShaderId(2),fragment:ShaderId(3),topology:PrimitiveTopology::TriangleList};
        r.register(p).unwrap();
        assert_eq!(r.register(p),Err(PipelineError::DuplicateId));
    }
    #[test] fn packet_requires_registered_pipeline() {
        let packet=DrawPacket{mesh:MeshHandle(atc_genesis_platform::AssetId(1)),material:MaterialId(atc_genesis_platform::AssetId(2)),pipeline:PipelineId(7)};
        assert_eq!(packet.validate(&RenderPipelineRegistry::default()),Err(PipelineError::InvalidShaderPair));
    }
}

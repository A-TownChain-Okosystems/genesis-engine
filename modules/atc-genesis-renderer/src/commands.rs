use atc_genesis_platform::{AssetId, EntityId, FrameId, Transform};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderCommandError {
    FrameAlreadyOpen,
    FrameNotOpen,
    InvalidEntity,
    NonFiniteTransform,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderCommand {
    BeginFrame(FrameId),
    SetCamera { position: [f32; 3], forward: [f32; 3] },
    Draw { entity: EntityId, mesh: AssetId, material: AssetId, transform: Transform },
    EndFrame,
}

#[derive(Default)]
pub struct RenderCommandStream {
    commands: Vec<RenderCommand>,
    frame_open: bool,
}

impl RenderCommandStream {
    pub fn begin_frame(&mut self, frame: FrameId) -> Result<(), RenderCommandError> {
        if self.frame_open { return Err(RenderCommandError::FrameAlreadyOpen); }
        self.commands.clear();
        self.commands.push(RenderCommand::BeginFrame(frame));
        self.frame_open = true;
        Ok(())
    }

    pub fn set_camera(&mut self, position: [f32; 3], forward: [f32; 3]) -> Result<(), RenderCommandError> {
        self.require_frame()?;
        if !position.iter().chain(forward.iter()).all(|v| v.is_finite()) {
            return Err(RenderCommandError::NonFiniteTransform);
        }
        self.commands.push(RenderCommand::SetCamera { position, forward });
        Ok(())
    }

    pub fn draw(&mut self, entity: EntityId, mesh: AssetId, material: AssetId, transform: Transform) -> Result<(), RenderCommandError> {
        self.require_frame()?;
        if entity.0 == 0 { return Err(RenderCommandError::InvalidEntity); }
        if !transform.translation.iter().chain(transform.rotation_xyzw.iter()).chain(transform.scale.iter()).all(|v| v.is_finite()) {
            return Err(RenderCommandError::NonFiniteTransform);
        }
        self.commands.push(RenderCommand::Draw { entity, mesh, material, transform });
        Ok(())
    }

    pub fn end_frame(&mut self) -> Result<(), RenderCommandError> {
        self.require_frame()?;
        self.commands.push(RenderCommand::EndFrame);
        self.frame_open = false;
        Ok(())
    }

    pub fn commands(&self) -> &[RenderCommand] { &self.commands }

    fn require_frame(&self) -> Result<(), RenderCommandError> {
        if self.frame_open { Ok(()) } else { Err(RenderCommandError::FrameNotOpen) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_lifecycle_is_enforced() {
        let mut stream = RenderCommandStream::default();
        assert_eq!(stream.end_frame(), Err(RenderCommandError::FrameNotOpen));
        stream.begin_frame(FrameId(1)).unwrap();
        assert_eq!(stream.begin_frame(FrameId(2)), Err(RenderCommandError::FrameAlreadyOpen));
        stream.draw(EntityId(1), AssetId(2), AssetId(3), Transform::default()).unwrap();
        stream.end_frame().unwrap();
        assert_eq!(stream.commands().len(), 3);
    }

    #[test]
    fn rejects_non_finite_draw_state() {
        let mut stream = RenderCommandStream::default();
        stream.begin_frame(FrameId(1)).unwrap();
        let transform = Transform { translation: [f32::NAN, 0.0, 0.0], ..Default::default() };
        assert_eq!(stream.draw(EntityId(1), AssetId(2), AssetId(3), transform), Err(RenderCommandError::NonFiniteTransform));
    }

    #[test]
    fn rejects_zero_entity() {
        let mut stream = RenderCommandStream::default();
        stream.begin_frame(FrameId(1)).unwrap();
        assert_eq!(stream.draw(EntityId(0), AssetId(2), AssetId(3), Transform::default()), Err(RenderCommandError::InvalidEntity));
    }
}

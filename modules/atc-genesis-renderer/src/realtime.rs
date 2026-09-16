//! Backend-agnostic real-time rendering primitives.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsBackend {
    Vulkan,
    Direct3D12,
    Metal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

impl Extent2D {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn is_valid(self) -> bool {
        self.width > 0 && self.height > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for ClearColor {
    fn default() -> Self {
        Self { r: 0.02, g: 0.02, b: 0.03, a: 1.0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderFrame {
    pub extent: Extent2D,
    pub clear: ClearColor,
}

impl RenderFrame {
    pub fn validate(&self) -> Result<(), RenderError> {
        if !self.extent.is_valid() {
            return Err(RenderError::InvalidExtent);
        }
        if [self.clear.r, self.clear.g, self.clear.b, self.clear.a]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(RenderError::InvalidColor);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    InvalidExtent,
    InvalidColor,
    BackendUnavailable,
}

pub trait RenderBackend {
    fn backend(&self) -> GraphicsBackend;
    fn begin_frame(&mut self, frame: &RenderFrame) -> Result<(), RenderError>;
    fn end_frame(&mut self) -> Result<(), RenderError>;
}

#[derive(Debug, Default)]
pub struct NullRenderer;

impl RenderBackend for NullRenderer {
    fn backend(&self) -> GraphicsBackend {
        GraphicsBackend::Vulkan
    }

    fn begin_frame(&mut self, frame: &RenderFrame) -> Result<(), RenderError> {
        frame.validate()
    }

    fn end_frame(&mut self) -> Result<(), RenderError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_extent() {
        let frame = RenderFrame { extent: Extent2D::new(0, 1080), clear: ClearColor::default() };
        assert_eq!(frame.validate(), Err(RenderError::InvalidExtent));
    }

    #[test]
    fn rejects_non_finite_color() {
        let frame = RenderFrame {
            extent: Extent2D::new(1920, 1080),
            clear: ClearColor { r: f32::NAN, ..ClearColor::default() },
        };
        assert_eq!(frame.validate(), Err(RenderError::InvalidColor));
    }
}

use atc_genesis_assets::MeshAsset;

/// Deterministic CPU reference framebuffer. Pixels are stored as tightly packed RGB8.
#[derive(Clone, Debug, PartialEq)]
pub struct SoftwareFramebuffer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    depth: Vec<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RasterCamera {
    /// Vertical field of view in radians.
    pub fov_y_radians: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for RasterCamera {
    fn default() -> Self {
        Self {
            fov_y_radians: std::f32::consts::FRAC_PI_3,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl SoftwareFramebuffer {
    pub const MAX_DIMENSION: u32 = 16_384;

    pub fn new(width: u32, height: u32, clear: [u8; 3]) -> Result<Self, String> {
        if width == 0 || height == 0 || width > Self::MAX_DIMENSION || height > Self::MAX_DIMENSION
        {
            return Err("framebuffer dimensions are outside the supported range".into());
        }
        let pixel_count = (width as usize)
            .checked_mul(height as usize)
            .ok_or_else(|| "framebuffer pixel count overflow".to_string())?;
        let byte_count = pixel_count
            .checked_mul(3)
            .ok_or_else(|| "framebuffer byte count overflow".to_string())?;
        let mut pixels = vec![0; byte_count];
        for pixel in pixels.as_chunks_mut::<3>().0 {
            pixel.copy_from_slice(&clear);
        }
        Ok(Self {
            width,
            height,
            pixels,
            depth: vec![f32::INFINITY; pixel_count],
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<[u8; 3]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let i = ((y * self.width + x) * 3) as usize;
        Some([self.pixels[i], self.pixels[i + 1], self.pixels[i + 2]])
    }

    /// Rasterizes camera-space triangles. The camera looks along negative Z.
    /// Triangles crossing the near/far plane are rejected rather than partially clipped.
    pub fn render_mesh(
        &mut self,
        mesh: &MeshAsset,
        camera: RasterCamera,
        color: [u8; 3],
    ) -> Result<usize, String> {
        mesh.validate()?;
        if !camera.fov_y_radians.is_finite()
            || camera.fov_y_radians <= 0.0
            || camera.fov_y_radians >= std::f32::consts::PI
            || !camera.near.is_finite()
            || !camera.far.is_finite()
            || camera.near <= 0.0
            || camera.far <= camera.near
        {
            return Err("invalid raster camera parameters".into());
        }

        let tan_half_fov = (camera.fov_y_radians * 0.5).tan();
        let aspect = self.width as f32 / self.height as f32;
        let mut triangles_drawn = 0;
        for triangle in mesh.indices.as_chunks::<3>().0 {
            let v = [
                mesh.vertices[triangle[0] as usize],
                mesh.vertices[triangle[1] as usize],
                mesh.vertices[triangle[2] as usize],
            ];
            let depths = [-v[0][2], -v[1][2], -v[2][2]];
            if depths
                .iter()
                .any(|d| !d.is_finite() || *d < camera.near || *d > camera.far)
            {
                continue;
            }
            let mut screen = [[0.0f32; 2]; 3];
            for i in 0..3 {
                let ndc_x = v[i][0] / (depths[i] * tan_half_fov * aspect);
                let ndc_y = v[i][1] / (depths[i] * tan_half_fov);
                screen[i] = [
                    (ndc_x + 1.0) * 0.5 * (self.width.saturating_sub(1) as f32),
                    (1.0 - ndc_y) * 0.5 * (self.height.saturating_sub(1) as f32),
                ];
            }
            if screen.iter().flatten().any(|p| !p.is_finite()) {
                continue;
            }
            let area = edge(screen[0], screen[1], screen[2]);
            if !area.is_finite() || area.abs() <= f32::EPSILON {
                continue;
            }

            let min_x = screen
                .iter()
                .map(|p| p[0])
                .fold(f32::INFINITY, f32::min)
                .floor()
                .max(0.0) as u32;
            let max_x = screen
                .iter()
                .map(|p| p[0])
                .fold(f32::NEG_INFINITY, f32::max)
                .ceil()
                .min((self.width - 1) as f32) as u32;
            let min_y = screen
                .iter()
                .map(|p| p[1])
                .fold(f32::INFINITY, f32::min)
                .floor()
                .max(0.0) as u32;
            let max_y = screen
                .iter()
                .map(|p| p[1])
                .fold(f32::NEG_INFINITY, f32::max)
                .ceil()
                .min((self.height - 1) as f32) as u32;
            if min_x > max_x || min_y > max_y {
                continue;
            }

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let point = [x as f32 + 0.5, y as f32 + 0.5];
                    let w0 = edge(screen[1], screen[2], point) / area;
                    let w1 = edge(screen[2], screen[0], point) / area;
                    let w2 = edge(screen[0], screen[1], point) / area;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                        continue;
                    }
                    let depth = w0 * depths[0] + w1 * depths[1] + w2 * depths[2];
                    let index = (y * self.width + x) as usize;
                    if depth < self.depth[index] {
                        self.depth[index] = depth;
                        let offset = index * 3;
                        self.pixels[offset..offset + 3].copy_from_slice(&color);
                    }
                }
            }
            triangles_drawn += 1;
        }
        Ok(triangles_drawn)
    }

    /// Transforms a validated world-space mesh through the canonical renderer Camera,
    /// then submits it to the same deterministic rasterization path.
    pub fn render_world_mesh(
        &mut self,
        mesh: &MeshAsset,
        camera: &crate::Camera,
        color: [u8; 3],
    ) -> Result<usize, String> {
        mesh.validate()?;
        let forward = normalize(camera.forward)
            .ok_or_else(|| "camera forward vector must be finite and non-zero".to_string())?;
        if !camera.position.iter().all(|v| v.is_finite()) {
            return Err("camera position must be finite".into());
        }
        let world_up = [0.0, 1.0, 0.0];
        let mut right = normalize(cross(forward, world_up));
        if right.is_none() {
            right = normalize(cross(forward, [0.0, 0.0, 1.0]));
        }
        let right = right.ok_or_else(|| "camera basis could not be constructed".to_string())?;
        let up = cross(right, forward);
        let mut camera_mesh = mesh.clone();
        for vertex in &mut camera_mesh.vertices {
            let relative = [
                vertex[0] - camera.position[0],
                vertex[1] - camera.position[1],
                vertex[2] - camera.position[2],
            ];
            *vertex = [
                dot(relative, right),
                dot(relative, up),
                -dot(relative, forward),
            ];
        }
        self.render_mesh(
            &camera_mesh,
            RasterCamera {
                fov_y_radians: camera.fov_y_radians,
                near: camera.near,
                far: camera.far,
            },
            color,
        )
    }

    /// Encodes a binary PPM (P6) image, avoiding platform-dependent encoders.
    pub fn to_ppm(&self) -> Vec<u8> {
        let mut output = format!("P6\n{} {}\n255\n", self.width, self.height).into_bytes();
        output.extend_from_slice(&self.pixels);
        output
    }
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(v: [f32; 3]) -> Option<[f32; 3]> {
    let length = dot(v, v).sqrt();
    if !length.is_finite() || length <= f32::EPSILON {
        None
    } else {
        Some([v[0] / length, v[1] / length, v[2] / length])
    }
}

fn edge(a: [f32; 2], b: [f32; 2], p: [f32; 2]) -> f32 {
    (p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangle() -> MeshAsset {
        MeshAsset {
            vertices: vec![[-0.7, -0.6, -2.0], [0.7, -0.6, -2.0], [0.0, 0.7, -2.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            uvs: vec![[0.0, 0.0]; 3],
            indices: vec![0, 1, 2],
        }
    }

    #[test]
    fn triangle_produces_pixels_and_ppm_artifact_bytes() {
        let mut frame = SoftwareFramebuffer::new(64, 64, [8, 12, 20]).unwrap();
        assert_eq!(
            frame
                .render_world_mesh(&triangle(), &crate::Camera::default(), [240, 80, 32])
                .unwrap(),
            1
        );
        assert_eq!(frame.pixel(32, 32), Some([240, 80, 32]));
        assert!(frame
            .pixels()
            .as_chunks::<3>()
            .0
            .iter()
            .any(|p| p == &[240, 80, 32]));
        let image = frame.to_ppm();
        assert!(image.starts_with(b"P6\n64 64\n255\n"));
        assert_eq!(image.len(), b"P6\n64 64\n255\n".len() + 64 * 64 * 3);
        assert_eq!(image, frame.to_ppm());
        if let Some(path) = std::env::var_os("ATC_RENDER_ARTIFACT_PATH") {
            let path = std::path::PathBuf::from(path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(path, &image).unwrap();
        }
    }

    #[test]
    fn nearer_triangle_wins_depth_test() {
        let mut frame = SoftwareFramebuffer::new(64, 64, [0, 0, 0]).unwrap();
        let near = triangle();
        let far = MeshAsset {
            vertices: vec![[-1.4, -1.2, -4.0], [1.4, -1.2, -4.0], [0.0, 1.4, -4.0]],
            ..triangle()
        };
        frame
            .render_mesh(&far, RasterCamera::default(), [20, 40, 200])
            .unwrap();
        frame
            .render_mesh(&near, RasterCamera::default(), [240, 80, 32])
            .unwrap();
        assert_eq!(frame.pixel(32, 32), Some([240, 80, 32]));
    }

    #[test]
    fn invalid_mesh_and_camera_are_rejected() {
        let mut frame = SoftwareFramebuffer::new(8, 8, [0, 0, 0]).unwrap();
        let mut bad = triangle();
        bad.indices = vec![0, 1, 99];
        assert!(frame
            .render_mesh(&bad, RasterCamera::default(), [255, 0, 0])
            .is_err());
        assert!(frame
            .render_mesh(
                &triangle(),
                RasterCamera {
                    near: 5.0,
                    far: 1.0,
                    ..Default::default()
                },
                [255, 0, 0]
            )
            .is_err());
    }

    #[test]
    fn framebuffer_rejects_unsafe_dimensions() {
        assert!(SoftwareFramebuffer::new(0, 10, [0, 0, 0]).is_err());
        assert!(
            SoftwareFramebuffer::new(SoftwareFramebuffer::MAX_DIMENSION + 1, 1, [0, 0, 0]).is_err()
        );
    }
}

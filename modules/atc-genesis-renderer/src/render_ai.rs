//! Deterministic Rendering AI policy layer.
//!
//! Rendering AI converts measured frame pressure, scene complexity and hardware
//! capabilities into a bounded render plan. It deliberately does not perform
//! model inference or mutate engine state. The plan is a pure decision artifact
//! that a concrete renderer can validate and execute.
//!
//! The policy is deterministic: identical inputs always produce identical plans.
//! This keeps render planning reproducible while leaving a future neural policy
//! behind the same contract.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderQuality {
    Performance,
    Balanced,
    Quality,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderSceneMetrics {
    pub visible_instances: u32,
    pub visible_triangles: u64,
    pub shadow_casters: u32,
    pub ray_traced_objects: u32,
    pub volumetric_layers: u32,
    pub texture_memory_mb: u32,
    pub gpu_frame_time_ms: f32,
    pub cpu_frame_time_ms: f32,
}

impl Default for RenderSceneMetrics {
    fn default() -> Self {
        Self {
            visible_instances: 0,
            visible_triangles: 0,
            shadow_casters: 0,
            ray_traced_objects: 0,
            volumetric_layers: 0,
            texture_memory_mb: 0,
            gpu_frame_time_ms: 0.0,
            cpu_frame_time_ms: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderHardwareProfile {
    pub compute: bool,
    pub ray_tracing: bool,
    pub mesh_shaders: bool,
    pub max_texture_memory_mb: u32,
    pub target_fps: u32,
}

impl Default for RenderHardwareProfile {
    fn default() -> Self {
        Self {
            compute: false,
            ray_tracing: false,
            mesh_shaders: false,
            max_texture_memory_mb: 4096,
            target_fps: 60,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderBudget {
    pub frame_time_ms: f32,
    pub min_resolution_scale: f32,
    pub max_resolution_scale: f32,
}

impl Default for RenderBudget {
    fn default() -> Self {
        Self {
            frame_time_ms: 1000.0 / 60.0,
            min_resolution_scale: 0.5,
            max_resolution_scale: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderPlan {
    pub quality: RenderQuality,
    pub resolution_scale: f32,
    pub lod_bias: f32,
    pub shadow_cascade_count: u8,
    pub ray_budget: u32,
    pub volumetric_layers: u32,
    pub enable_gi: bool,
    pub enable_reflections: bool,
    pub enable_neural_reconstruction: bool,
    pub enable_frame_generation: bool,
    pub aggressive_culling: bool,
}

impl RenderPlan {
    fn clamp(mut self, budget: RenderBudget) -> Self {
        self.resolution_scale = self
            .resolution_scale
            .clamp(budget.min_resolution_scale, budget.max_resolution_scale);
        self.lod_bias = self.lod_bias.clamp(-1.0, 2.0);
        self.shadow_cascade_count = self.shadow_cascade_count.clamp(1, 4);
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderingAi;

impl RenderingAi {
    pub fn plan(
        &self,
        metrics: RenderSceneMetrics,
        hardware: RenderHardwareProfile,
        budget: RenderBudget,
        requested_quality: RenderQuality,
    ) -> RenderPlan {
        let frame_time = metrics
            .gpu_frame_time_ms
            .max(metrics.cpu_frame_time_ms)
            .max(0.0);
        let over_budget = frame_time > budget.frame_time_ms;
        let severe_pressure = frame_time > budget.frame_time_ms * 1.25;
        let memory_pressure = hardware.max_texture_memory_mb > 0
            && metrics.texture_memory_mb.saturating_mul(100)
                >= hardware.max_texture_memory_mb.saturating_mul(90);

        let scene_pressure = metrics.visible_instances >= 20_000
            || metrics.visible_triangles >= 10_000_000
            || metrics.shadow_casters >= 2_000;

        let quality = if severe_pressure {
            RenderQuality::Performance
        } else if over_budget || memory_pressure || scene_pressure {
            match requested_quality {
                RenderQuality::Quality => RenderQuality::Balanced,
                _ => RenderQuality::Performance,
            }
        } else {
            requested_quality
        };

        let mut plan = match quality {
            RenderQuality::Performance => RenderPlan {
                quality,
                resolution_scale: 0.70,
                lod_bias: 1.0,
                shadow_cascade_count: 2,
                ray_budget: 0,
                volumetric_layers: metrics.volumetric_layers.min(8),
                enable_gi: hardware.compute && !severe_pressure,
                enable_reflections: hardware.ray_tracing && !severe_pressure,
                enable_neural_reconstruction: hardware.compute,
                enable_frame_generation: hardware.compute && hardware.target_fps >= 60,
                aggressive_culling: true,
            },
            RenderQuality::Balanced => RenderPlan {
                quality,
                resolution_scale: 0.85,
                lod_bias: 0.5,
                shadow_cascade_count: 3,
                ray_budget: if hardware.ray_tracing { 2 } else { 0 },
                volumetric_layers: metrics.volumetric_layers.min(16),
                enable_gi: hardware.compute,
                enable_reflections: hardware.ray_tracing,
                enable_neural_reconstruction: hardware.compute,
                enable_frame_generation: false,
                aggressive_culling: scene_pressure,
            },
            RenderQuality::Quality => RenderPlan {
                quality,
                resolution_scale: 1.0,
                lod_bias: 0.0,
                shadow_cascade_count: 4,
                ray_budget: if hardware.ray_tracing { 4 } else { 0 },
                volumetric_layers: metrics.volumetric_layers.min(32),
                enable_gi: hardware.compute,
                enable_reflections: hardware.ray_tracing,
                enable_neural_reconstruction: hardware.compute,
                enable_frame_generation: false,
                aggressive_culling: false,
            },
        };

        if memory_pressure {
            plan.lod_bias += 0.5;
            plan.volumetric_layers = plan.volumetric_layers.min(8);
        }

        if severe_pressure {
            plan.resolution_scale = budget.min_resolution_scale;
            plan.shadow_cascade_count = 1;
            plan.ray_budget = 0;
            plan.enable_reflections = false;
            plan.enable_gi = false;
            plan.volumetric_layers = plan.volumetric_layers.min(4);
            plan.enable_frame_generation = false;
            plan.aggressive_culling = true;
        }

        plan.clamp(budget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hardware() -> RenderHardwareProfile {
        RenderHardwareProfile {
            compute: true,
            ray_tracing: true,
            mesh_shaders: true,
            max_texture_memory_mb: 8192,
            target_fps: 60,
        }
    }

    #[test]
    fn planning_is_deterministic() {
        let ai = RenderingAi;
        let metrics = RenderSceneMetrics {
            visible_instances: 1000,
            visible_triangles: 1_000_000,
            gpu_frame_time_ms: 12.0,
            cpu_frame_time_ms: 8.0,
            ..Default::default()
        };
        let a = ai.plan(metrics, hardware(), RenderBudget::default(), RenderQuality::Quality);
        let b = ai.plan(metrics, hardware(), RenderBudget::default(), RenderQuality::Quality);
        assert_eq!(a, b);
    }

    #[test]
    fn severe_gpu_pressure_disables_expensive_features() {
        let ai = RenderingAi;
        let metrics = RenderSceneMetrics {
            gpu_frame_time_ms: 30.0,
            cpu_frame_time_ms: 10.0,
            ..Default::default()
        };
        let plan = ai.plan(metrics, hardware(), RenderBudget::default(), RenderQuality::Quality);
        assert_eq!(plan.quality, RenderQuality::Performance);
        assert_eq!(plan.ray_budget, 0);
        assert!(!plan.enable_gi);
        assert!(!plan.enable_reflections);
        assert!(plan.aggressive_culling);
    }

    #[test]
    fn quality_uses_hardware_capabilities() {
        let ai = RenderingAi;
        let plan = ai.plan(
            RenderSceneMetrics {
                volumetric_layers: 64,
                ..Default::default()
            },
            hardware(),
            RenderBudget::default(),
            RenderQuality::Quality,
        );
        assert_eq!(plan.ray_budget, 4);
        assert!(plan.enable_gi);
        assert!(plan.enable_reflections);
        assert_eq!(plan.volumetric_layers, 32);
    }

    #[test]
    fn memory_pressure_increases_lod_bias() {
        let ai = RenderingAi;
        let mut hw = hardware();
        hw.max_texture_memory_mb = 1000;
        let metrics = RenderSceneMetrics {
            texture_memory_mb: 950,
            ..Default::default()
        };
        let plan = ai.plan(metrics, hw, RenderBudget::default(), RenderQuality::Quality);
        assert!(plan.lod_bias > 0.0);
        assert!(plan.volumetric_layers <= 8);
    }

    #[test]
    fn budget_bounds_resolution_scale() {
        let ai = RenderingAi;
        let budget = RenderBudget {
            frame_time_ms: 16.0,
            min_resolution_scale: 0.6,
            max_resolution_scale: 0.9,
        };
        let plan = ai.plan(
            RenderSceneMetrics::default(),
            hardware(),
            budget,
            RenderQuality::Quality,
        );
        assert_eq!(plan.resolution_scale, 0.9);
    }
}

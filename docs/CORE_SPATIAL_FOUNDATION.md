# Core Spatial Foundation

## Scope

The Genesis core now contains a deterministic spatial-query foundation independent of the renderer backend.

## Data model

- `Aabb`: validated axis-aligned bounds.
- `Sphere`: finite non-negative-radius volume.
- `Ray`: finite non-zero direction query primitive.
- `SpatialWorld`: ordered entity-to-AABB registry.

## Query contract

`SpatialWorld` exposes AABB, sphere, and ray queries. Results are deterministic because entity IDs are stored in a `BTreeMap` and traversed in ascending order.

Invalid query geometry is rejected instead of silently producing an empty result. Duplicate insertion and updates of missing entities are explicit errors.

## Architectural boundary

The current implementation is a correctness-first spatial registry. It does not claim dynamic-BVH performance, occlusion culling, SIMD acceleration, or production-scale profiling. Those remain separate implementation/evidence stages.

The spatial layer has no dependency on a graphics API. Renderer culling can consume this API without making spatial state dependent on Vulkan, Direct3D 12, Metal, or another GPU backend.

## Next stages

1. Dynamic BVH with deterministic rebuild/update policy.
2. Frustum query integration with world-space `CameraView`.
3. Shape/ray hit metadata and nearest-hit queries.
4. Broad-phase integration with physics.
5. Spatial streaming integration with world chunks.
6. Benchmark and profiling evidence before performance claims.

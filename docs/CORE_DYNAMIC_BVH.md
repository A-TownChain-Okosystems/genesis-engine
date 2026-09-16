# Dynamic BVH Foundation

## Purpose

`DynamicBvh` provides a deterministic mutable spatial index for entities whose AABB changes during runtime.

## Current contract

- Entity IDs are kept in ascending order.
- Insert rejects duplicate entities.
- Update rejects missing entities and invalid bounds.
- Remove rejects missing entities.
- AABB queries return ascending entity IDs.
- The implementation is correctness-first and intentionally independent of graphics APIs.

## Current limitation

This is the first mutable spatial-index stage, not yet a hierarchical dynamic BVH with tree rotations, fat AABBs, incremental refitting, or broad-phase balancing. The API is kept narrow so a hierarchical implementation can replace the internal storage without changing query ownership.

## Integration targets

1. Replace the ordered leaf registry with a true dynamic hierarchy.
2. Add deterministic reinsertion/refit policy.
3. Connect `SpatialWorld` and `DynamicBvh` through one spatial authority.
4. Add frustum, sphere, ray and nearest-hit query APIs.
5. Add profiling evidence before making performance claims.

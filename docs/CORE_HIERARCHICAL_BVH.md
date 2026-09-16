# Hierarchical BVH Transition

## Status

The core now exposes `HierarchicalBvh` as a stable query-facing abstraction, while the current implementation remains a deterministic ordered leaf index. This deliberately separates the public spatial contract from the future tree implementation.

## Contract

- Entity identity is stable and ordered.
- Insert, update and remove are explicit operations.
- AABB, sphere and ray queries reject invalid input.
- Query output is deterministic and sorted by entity ID.
- No graphics backend is required.

## Not yet claimed

The current implementation does **not** claim logarithmic broad-phase complexity, tree balancing, rotations, fat AABBs, incremental refitting, SIMD acceleration, multithreaded traversal, or measured performance targets.

## Next implementation

The internal representation will be replaced with a real hierarchy using deterministic split selection, parent/child nodes, refit propagation and a bounded update policy. The public query contract should remain stable during that migration.

## Evidence policy

Correctness is established by source-level invariants and unit tests once executed. Performance claims require benchmark evidence. Production readiness additionally requires CI, platform validation, security review and governance conformance evidence.

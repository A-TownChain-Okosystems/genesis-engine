# Genesis Physics Foundation

## Scope

The physics module provides a deterministic fixed-step simulation foundation and a backend-independent collision layer.

## Simulation

`PhysicsSimulation` currently provides:

- fixed timestep accumulation;
- bounded substeps;
- dynamic-body integration;
- kinematic-body integration;
- static bodies;
- configurable gravity;
- optional floor constraint;
- restitution handling;
- ECS transform synchronization;
- world-chunk collision bridging.

## Collision pipeline

The collision layer provides:

1. validated AABB colliders;
2. deterministic collider ordering by `EntityId`;
3. broad-phase AABB pair generation;
4. deterministic canonical collision pairs;
5. AABB queries;
6. raycasts with nearest-hit selection;
7. point-resolution information.

Broad-phase pairs are a candidate set only. They are not narrow-phase contacts and must not be treated as final collision manifolds.

## Determinism

Entity ordering and collision-pair canonicalization are explicit. The fixed-step accumulator bounds simulation work per frame. Parallel execution and floating-point determinism remain separate engineering concerns and require dedicated evidence.

## Current limitations

This foundation does **not** yet claim:

- a production rigid-body solver;
- continuous collision detection;
- convex/GJK/EPA narrow phase;
- triangle-mesh collision;
- joints/constraints;
- sleeping/islands;
- articulated bodies;
- SIMD acceleration;
- deterministic cross-platform floating-point equivalence;
- production performance.

## Next implementation stages

- dynamic BVH broad phase integration;
- sphere/capsule/OBB/convex narrow phase;
- contact manifolds and sequential impulse solver;
- friction and restitution constraints;
- CCD;
- joints and constraints;
- sleeping/island management;
- physics queries shared with gameplay and AI;
- profiling and conformance evidence.

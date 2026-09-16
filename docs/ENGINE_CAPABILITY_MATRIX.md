# Genesis Engine Capability Matrix

## Purpose

This document defines the target capability surface for Genesis Engine as an independent, general-purpose game engine. It is an implementation inventory, not a claim that all capabilities are production-ready.

## Architectural boundary

Genesis Engine is a native engine stack. It does not depend on Unreal Engine or another external game engine.

```text
Genesis Project
    ↓
Genesis Editor / CLI / SDK
    ↓
Genesis Runtime
    ├── Core / ECS / Jobs / Memory
    ├── World / Scene / Streaming / Spatial Acceleration
    ├── Renderer / GPU Abstraction / Native Backends
    ├── Physics
    ├── Animation
    ├── Audio
    ├── Gameplay / Input
    ├── AI
    ├── Networking
    └── Asset Runtime
```

A-TownChain integration is an explicit subsystem boundary. Chain state must not depend on a graphics backend.

## Capability inventory

| Domain | Required capabilities | Target state |
|---|---|---|
| Core | ECS, jobs, memory, events, handles, time, logging, profiling, configuration, modules | Foundation / expand |
| World | transforms, scenes, prefabs, levels, streaming, LOD, spatial partitioning, serialization | Partial / expand |
| Spatial | AABB, bounding spheres, BVH, grid/octree, frustum, distance, occlusion culling, queries | AABB/sphere/frustum foundation |
| Renderer | resources, buffers, textures, samplers, bindings, shaders, pipelines, render graph, synchronization | Foundation / expand |
| Lighting | PBR, shadows, cascaded shadows, IBL, HDR, post processing | Planned |
| GPU | Vulkan, Direct3D 12, Metal native backends | Planned |
| Assets | import, validation, intermediate data, cooking, registry, dependencies, cache, hot reload | Planned / partial modules |
| Physics | world, bodies, shapes, broad/narrow phase, solver, joints, queries, triggers | Planned / module foundation |
| Animation | skeletons, clips, state machines, blending, IK, retargeting, root motion, morphs | Module / expand |
| Audio | device, assets, streaming, spatial audio, mixer, effects, reverb, music, voice | Module / expand |
| Gameplay | input, camera, character, interaction, tags, timers, state machines, save/game modes | Module / expand |
| AI | behavior trees, utility, navigation, pathfinding, steering, perception, goals, debugger | Module / expand |
| Networking | transport, connections, serialization, replication, authority, prediction, reconciliation, snapshots, RPC, security | Module / expand |
| Editor | project, scene/world, hierarchy, inspector, viewport, assets, materials, shaders, animation, debugging, profiling | Planned / expand |
| Tools | CLI, project generator, cooker, validator, shader compiler, packaging, build/dependency graphs, generators | Planned / expand |
| SDK | core, ECS, world, renderer, physics, audio, animation, input, gameplay, AI, network, asset, editor APIs | Planned / expand |
| Platform | Windows, Linux, macOS; later Android/iOS; window/input/filesystem/thread/GPU/audio/native integration | Abstraction / expand |
| Build | dependency resolution, cooking, shader compilation, build, packaging, bundles, patching, versioning | Planned / expand |
| Developer experience | hot reload, console, logging, assertions, debug draw, CPU/GPU/memory profiling, capture, crash dumps | Partial / planned |
| Governance | architecture, contracts, formats, serialization, compatibility, determinism, security, reproducibility | Governance integration |

## Runtime dependency direction

The intended dependency direction is:

```text
Platform
  ↓
Core
  ↓
ECS / World / Assets
  ↓
Physics / Animation / Audio / Gameplay / AI / Network
  ↓
Runtime
  ↓
Renderer
```

Subsystems should depend on stable lower-level contracts rather than concrete editor implementations. Editor and tools consume runtime/SDK interfaces; runtime does not depend on the editor.

## Renderer implementation target

```text
Renderer API
    ↓
GPU Abstraction
    ├── Vulkan
    ├── Direct3D 12
    └── Metal
```

The renderer owns presentation. A-TownChain owns authoritative chain state and ownership semantics. This boundary prevents rendering concerns from becoming consensus dependencies.

## Asset pipeline target

```text
Source Asset
    ↓
Importer
    ↓
Validator
    ↓
Intermediate Asset
    ↓
Cooker
    ↓
Runtime Asset
    ↓
Registry / Dependency Graph / Cache
    ↓
Streaming / Hot Reload / Packaging
```

## Build pipeline target

```text
Project
  ↓
Dependency Resolution
  ↓
Asset Cooking + Shader Compilation
  ↓
Code Build
  ↓
Packaging
  ↓
Runtime Bundle
```

Build profiles must be explicit: development, debug, release, and shipping.

## Current implementation focus

The immediate technical sequence is:

1. Spatial acceleration: BVH + spatial queries + hierarchical culling.
2. GPU resource model and validation.
3. Shader/pipeline resource contracts.
4. Native renderer command/resource integration.
5. Asset registry and dependency graph.
6. Runtime subsystem orchestration.
7. Physics/animation/audio/input foundations.
8. Editor viewport and inspection foundations.
9. SDK and CLI integration.
10. Native GPU backends and production-grade tooling.

## Evidence policy

Implementation existence, unit tests, CI results, security audits, and production-readiness are separate evidence classes. A feature is not considered production-ready solely because source code or tests exist.

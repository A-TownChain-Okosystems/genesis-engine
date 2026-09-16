# Genesis Engine Extension Roadmap

## Scope

This roadmap turns the Genesis Engine capability matrix into an implementation sequence. It distinguishes implemented foundations from planned capability and does not treat source existence as proof of CI success or production readiness.

## P0 — shared runtime foundation

### Core runtime
- [x] Time/frame state foundation
- [x] Event/message bus foundation
- [x] Generational ID primitive
- [x] Runtime module registry foundation
- [x] Diagnostics/log records
- [x] Trace span foundation
- [ ] Generational storage/handle validation
- [ ] Job system and worker scheduler
- [ ] Memory allocator abstraction
- [ ] Runtime configuration
- [ ] Plugin lifecycle manager

### Spatial runtime
- [x] AABB
- [x] Bounding sphere
- [x] Static BVH build
- [x] Deterministic frustum query
- [ ] Dynamic BVH updates
- [ ] SpatialWorld ownership layer
- [ ] AABB/sphere/ray/shape queries
- [ ] Spatial grid
- [ ] Octree where workload requires it
- [ ] Occlusion hierarchy

### Renderer GPU foundation
- [x] Render graph foundation
- [x] Render commands
- [x] Pipeline registry foundation
- [x] Mesh/material resource handles
- [ ] GPU resource manager
- [ ] Buffer/texture/sampler descriptors
- [ ] Descriptor/binding model
- [ ] Resource lifetime/state tracking
- [ ] GPU allocation
- [ ] Upload/readback queues
- [ ] Shader source/compiler/reflection
- [ ] Shader permutation cache
- [ ] Pipeline cache

### Assets
- [ ] Stable AssetId policy
- [ ] Asset registry
- [ ] Dependency graph
- [ ] Importer contracts
- [ ] Validator contracts
- [ ] Intermediate asset format
- [ ] Deterministic cooker
- [ ] Incremental cooking
- [ ] Cache invalidation
- [ ] Runtime manifest
- [ ] Streaming integration
- [ ] Hot reload

## P1 — engine subsystems

### World
- [ ] Transform hierarchy
- [ ] Scene graph/ECS scene model
- [ ] Prefabs
- [ ] Levels
- [ ] LOD
- [ ] Serialization
- [ ] Save/load
- [ ] Dynamic spatial structures

### Physics
- [ ] Physics world
- [ ] Static/rigid bodies
- [ ] Collision shapes
- [ ] Broad phase
- [ ] Narrow phase
- [ ] Contact solver
- [ ] Constraints/joints
- [ ] Raycast/shape cast
- [ ] Triggers
- [ ] Layers/masks
- [ ] Deterministic mode
- [ ] Character controller
- [ ] Debug visualization

### Animation
- [ ] Skeleton
- [ ] Clips/player
- [ ] State machine
- [ ] Blend trees
- [ ] IK
- [ ] Retargeting
- [ ] Root motion
- [ ] Animation events
- [ ] Morph targets
- [ ] GPU skinning
- [ ] Compression

### Audio
- [ ] Device abstraction
- [ ] Audio resource manager
- [ ] Streaming
- [ ] Spatial audio/listener
- [ ] Mixer/buses
- [ ] Effects/reverb
- [ ] Music/voice
- [ ] Debugging

### Gameplay
- [ ] Input/gameplay integration
- [ ] Character framework
- [ ] Interaction
- [ ] Gameplay tags
- [ ] Timers/state machines
- [ ] Spawn system
- [ ] Save game
- [ ] Game modes
- [ ] Native Gameplay SDK

### AI
- [ ] Behavior trees
- [ ] Utility AI
- [ ] Navigation/NavMesh
- [ ] Pathfinding
- [ ] Steering
- [ ] Perception
- [ ] Squad/group AI
- [ ] AI debugger
- [ ] Explicit Aurora AI API boundary

### Networking
- [ ] Transport abstraction
- [ ] UDP/QUIC implementation
- [ ] Connection state machine
- [ ] Authenticated sessions
- [ ] Replication graph
- [ ] Networked ECS
- [ ] Authority model
- [ ] Client prediction/reconciliation
- [ ] Interpolation
- [ ] Lag compensation
- [ ] Snapshots/RPC
- [ ] Rate limiting/security

## P1 — editor, tools and SDK

### Editor
- [ ] Project manager
- [ ] Scene/world editor
- [ ] Hierarchy
- [ ] Entity/component inspector
- [ ] Viewport
- [ ] Asset browser
- [ ] Material/shader editor
- [ ] Animation editor
- [ ] Physics/audio/AI debugging
- [ ] Profiler/console
- [ ] Build/package manager

### Tools
- [ ] Project generator
- [ ] Asset cooker/validator
- [ ] Shader compiler
- [ ] Package builder
- [ ] Build graph
- [ ] Dependency graph
- [ ] Code/doc generator
- [ ] Test/benchmark runner
- [ ] Crash/symbol tooling

### SDK
- [ ] Core/ECS/World APIs
- [ ] Renderer/Physics/Audio APIs
- [ ] Animation/Input/Gameplay APIs
- [ ] AI/Network/Asset APIs
- [ ] Editor APIs

## P2 — platform and distribution

- [ ] Windows platform implementation
- [ ] Linux platform implementation
- [ ] macOS platform implementation
- [ ] Android/iOS extension point
- [ ] Window/input/filesystem/thread/timer APIs
- [ ] Dynamic libraries
- [ ] GPU/audio/clipboard/native dialogs
- [ ] Development/Debug/Release/Shipping profiles
- [ ] Asset bundles
- [ ] Patching
- [ ] Versioning
- [ ] Crash reporting

## Rendering completion

After the GPU resource/shader foundations:

- [ ] PBR
- [ ] Shadow maps
- [ ] Cascaded shadows
- [ ] IBL
- [ ] HDR
- [ ] Tone mapping
- [ ] Bloom
- [ ] SSAO
- [ ] TAA/temporal upscaling
- [ ] MSAA
- [ ] Post processing
- [ ] Debug renderer
- [ ] GPU profiler
- [ ] Vulkan backend
- [ ] Direct3D 12 backend
- [ ] Metal backend

## Governance and conformance

Each major subsystem should eventually provide:

1. Architecture contract
2. Public API contract
3. Validation rules
4. Unit/integration tests
5. Determinism requirements where applicable
6. Security requirements
7. Evidence artifacts
8. Conformance mapping to `atc-standards`

## Current implementation status

The branch contains renderer/culling foundations and now a first shared core-runtime module. CI, workflow execution and merge status are intentionally not inferred from source changes. Production readiness remains unestablished until the required verification evidence exists.

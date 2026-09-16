# Genesis Real-Time Engine Architecture

## Boundary

Genesis Engine is a fully independent general-purpose real-time game and rendering engine. Rendering, simulation, gameplay, editor, runtime and SDK capabilities are implemented by Genesis itself. No external game engine is a runtime dependency.

```text
Genesis Engine
  ├─ Core / ECS / Jobs / Memory
  ├─ World / Streaming / Terrain
  ├─ Renderer / GPU abstraction / Backends
  ├─ Materials / Shaders / Lighting
  ├─ Physics / Animation / Audio
  ├─ Gameplay / AI / Input
  ├─ Assets / Import / Cook / Packaging
  ├─ Editor / Profiler / Debugger
  ├─ Runtime / Platform / Build
  └─ SDK / A-TownChain integration
```

A-TownChain integration is an explicit boundary for identity, ownership, assets, worlds and contracts. The authoritative chain does not depend on GPU execution, editor state or host rendering APIs.

## Rendering contract

`atc-genesis-renderer` exposes backend-independent frame primitives. Platform GPU implementations are isolated behind `RenderBackend`; no consensus or ownership logic may depend on a GPU backend.

Backend targets are Vulkan, Direct3D 12 and Metal, with WebGPU/OpenGL available as non-authoritative platform targets where appropriate. The current implementation provides deterministic render graph construction, command ordering, camera culling and a null backend. Concrete GPU backends are implemented inside Genesis Engine behind the same contract.

## Game integration contract

The runtime owns the complete game loop and integrates ECS, world streaming, physics, animation, audio, gameplay, networking and rendering directly. No Unreal Engine adapter is part of the architecture.

The integration flow is:

```text
Input
  -> Gameplay / AI
  -> ECS / World
  -> Physics / Animation
  -> Audio / Network
  -> Render Graph
  -> Genesis GPU Backend
  -> Native Platform Runtime
```

The editor is a Genesis subsystem and targets the same runtime/renderer contracts. Games built with Genesis consume the Genesis SDK rather than another engine's SDK.

## Determinism and security

Rendering is presentation state. It must not silently mutate authoritative blockchain state. Network, wallet, DAO and smart-contract operations remain explicit integration APIs with authentication and authorization at their respective boundaries.

Host time, randomness and device-specific behavior must not enter consensus-critical execution. Where the engine requires wall-clock time, entropy or GPU-specific behavior, those values remain outside authoritative deterministic state transitions.

## Roadmap

1. Renderer resource model and command abstraction.
2. Vulkan backend.
3. Direct3D 12 backend.
4. Metal backend.
5. PBR/material/shader pipeline.
6. GPU culling, shadows, HDR, temporal processing and post-processing.
7. World streaming, spatial partitioning and terrain.
8. Native editor viewport and asset pipeline.
9. Native game runtime, packaging and platform launchers.
10. Genesis SDK and A-TownChain integration.
11. Performance benchmarks and conformance gates.

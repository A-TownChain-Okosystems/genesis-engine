# Genesis Real-Time Engine Architecture

## Boundary

Genesis Engine remains an independent general-purpose engine. Blockchain, ownership, DAO, franchise and authoritative world state are not delegated to an external renderer.

```text
Genesis Engine
  ├─ ECS / World / Gameplay
  ├─ Assets / Materials
  ├─ Physics / Animation / Audio
  ├─ Renderer abstraction
  ├─ Runtime / Build / SDK
  └─ Genesis state & ownership integration
       │
       ├─ Native renderer backends
       ├─ Unreal integration adapter
       └─ Web visualization adapter
```

## Rendering contract

`atc-genesis-renderer` exposes backend-independent frame primitives. Platform GPU implementations must be isolated behind `RenderBackend`; no consensus or ownership logic may depend on a GPU backend.

Initial backend targets are Vulkan, Direct3D 12 and Metal. The first implementation provides validation and a deterministic null backend; platform GPU execution is a subsequent implementation phase.

## Determinism and security

Rendering is presentation state. It must not silently mutate authoritative blockchain state. Network, wallet, DAO and smart-contract operations remain explicit integration APIs with authentication and authorization at their respective boundaries.

## Roadmap

1. Renderer resource model and command abstraction.
2. Vulkan backend.
3. D3D12 backend.
4. Metal backend.
5. PBR/material/shader pipeline.
6. GPU culling, shadows, HDR and post-processing.
7. World streaming and spatial partitioning.
8. Editor viewport and asset pipeline integration.
9. Unreal SDK adapter and Web renderer adapter.

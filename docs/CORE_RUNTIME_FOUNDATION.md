# Genesis Core Runtime Foundation

## Purpose

`atc-genesis-core` is the shared foundation for runtime-independent engine services. It must remain free of renderer, physics, networking and platform dependencies.

## Implemented foundations

### Time

`TimeState` provides monotonically increasing frame identifiers and accumulated elapsed time. Invalid and negative frame deltas are rejected.

### Handles

`GenerationalId` defines the common identity primitive. `GenerationalStorage<T>` provides insertion, lookup and removal with stale-generation rejection.

### Memory

`LinearAllocator` provides a bounded deterministic allocation primitive with power-of-two alignment and explicit reset semantics. It is a foundation, not a replacement for platform-specific virtual memory or general-purpose allocation.

### Jobs

`JobQueue<T>` provides deterministic FIFO scheduling with explicit capacity and monotonic job identifiers. Parallel worker execution is intentionally a later layer so scheduling semantics remain independently testable.

### Events

`EventBus` provides ordered event publication and drain semantics. Topics are required to be non-empty.

### Modules

`ModuleRegistry` provides deterministic module registration and duplicate-ID rejection. A complete lifecycle manager is still pending.

### Diagnostics and tracing

`Diagnostics` stores structured log-level/code/message records. `Tracer` provides nested LIFO trace spans. Exporters, timestamps, thread attribution and CPU/GPU correlation remain future work.

## Architectural rules

1. Core must not depend on GPU APIs.
2. Core must not depend on a particular operating system.
3. Handles must reject stale generations.
4. Runtime scheduling must have deterministic semantics independent of worker count.
5. Diagnostics must be usable before renderer initialization.
6. Plugin lifecycle must be explicit and fail visibly.

## Remaining core work

- production worker scheduler
- dependency-aware job graph
- memory arenas/pools and allocation statistics
- handle pools with generation-overflow policy
- runtime configuration service
- plugin state machine
- structured tracing sinks/exporters
- reflection/RTTI metadata registry
- crash/diagnostic integration

## Verification status

Unit tests have been authored alongside the foundations. CI execution has not been claimed by this document. Production readiness remains `NOT_ESTABLISHED` until repository verification and integration evidence are available.

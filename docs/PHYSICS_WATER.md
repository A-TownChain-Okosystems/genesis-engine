# Water Simulation Foundation

## Scope

Genesis now contains a deterministic, CPU-side water-surface foundation in `atc-genesis-physics`.

The implementation models a regular height field with per-cell vertical velocity and a small foam accumulator. It is intended as a simulation foundation and API contract, not as a production ocean renderer or physically complete fluid solver.

## Current API

- `WaterConfig`
- `WaterSurface`
- `WaterSample`
- `WaterBody`
- deterministic disturbance and height updates
- fixed-step-compatible surface integration
- bounded damping
- deterministic foam accumulation
- basic buoyancy force calculation

## Simulation model

The surface is represented as a regular grid. Each cell stores:

- height
- vertical velocity
- foam amount

A disturbance changes the local height. Each simulation step estimates a discrete Laplacian from the four neighboring cells and applies a damped wave response.

This is a correctness-oriented height-field model. It does not claim incompressible Navier-Stokes simulation.

## Determinism

The grid is traversed in stable `z`/`x` order. Every update reads from a snapshot of the previous state before writing the next state. This avoids order-dependent neighbor updates and makes identical inputs produce identical results within the same floating-point execution model.

## Integration targets

Future engine integration should connect water to:

1. world/terrain height fields;
2. collision and buoyancy;
3. renderer displacement and normal generation;
4. shoreline and shallow-water classification;
5. reflection/refraction and screen-space effects;
6. precipitation and rivers;
7. ocean-scale spectral waves;
8. GPU simulation where profiling establishes a requirement.

## Explicit limitations

The current implementation does not provide:

- full 3D fluid volume simulation;
- incompressible pressure projection;
- adaptive grids;
- SPH or FLIP/PIC particles;
- FFT ocean spectra;
- physically validated wave spectra;
- GPU compute implementation;
- production performance evidence.

Those capabilities require separate specifications, implementations, tests and evidence.

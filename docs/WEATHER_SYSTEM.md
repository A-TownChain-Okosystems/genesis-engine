# Genesis Engine Weather System

## Status

**Foundation implemented — not yet production-ready meteorological simulation.**

The weather system is a deterministic simulation layer for the Genesis Engine. It owns weather state that can drive rendering, atmosphere, clouds, precipitation, visibility, audio and gameplay without coupling those systems to external weather services.

## Scope

The weather field currently represents, per spatial cell:

- temperature in °C
- relative humidity in `[0, 1]`
- pressure in hPa
- horizontal wind vector
- precipitation intensity in `[0, 1]`
- cloud-cover factor in `[0, 1]`
- visibility in metres

A deterministic update step performs stable scalar/vector diffusion and bounded precipitation/cloud evolution.

## Architecture

```text
WeatherField
├── WeatherConfig
├── WeatherCell[]
├── deterministic update
├── WeatherClass classification
└── renderer/environment consumers
      ├── Sky / Atmosphere
      ├── Clouds
      ├── Fog / Visibility
      ├── Rain / Snow
      ├── Ocean / Water
      └── Audio / Gameplay
```

Weather is simulation state. Rendering consumes weather state but does not become authoritative over it.

## Determinism

The current implementation uses only deterministic arithmetic and fixed iteration order. It does not query network services or wall-clock weather APIs. This makes the foundation suitable for replay and synchronized simulation, subject to the normal floating-point determinism policy of the target platform.

## Weather classes

The classifier exposes:

- `Clear`
- `Cloudy`
- `Overcast`
- `Rain`
- `Snow`
- `Storm`

The classes are presentation/gameplay categories, not meteorological forecasts.

## Current limitations

This foundation is **not** a physically complete atmosphere/weather solver. Missing production layers include:

1. pressure/temperature thermodynamic coupling
2. humidity phase changes and condensation physics
3. 3D atmospheric advection
4. turbulence and boundary-layer modelling
5. precipitation particle/microphysics model
6. cloud volume simulation and cloud shadows
7. lightning/electrical storm model
8. terrain-aware wind and orographic precipitation
9. high-performance spatial advection using GPU compute
10. authoritative network replication/interpolation policy
11. coupling to the completed sky/atmosphere renderer
12. coupling to ocean/3D-fluid boundary conditions

These are explicit follow-up implementation items rather than hidden assumptions.

## Verification

Unit tests cover deterministic updates and state clamping. CI execution is intentionally deferred with the rest of the project's CI/merge phase.

## Integration rule

Weather must remain independent from A-TownChain consensus. A-TownChain may provide authoritative world/time/game-state inputs, while weather remains an engine simulation subsystem with explicit synchronization interfaces.

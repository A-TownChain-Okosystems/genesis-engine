# Genesis Engine Vehicle System

## Scope

Genesis Engine vehicles are simulated independently from rendering and networking and support three explicit physical domains:

- **Ground** — traction/drive force, braking, rolling resistance, aerodynamic drag and gravity.
- **Water** — thrust, longitudinal/lateral hydrodynamic drag, buoyancy, waterline and environmental wind.
- **Air** — thrust, dynamic pressure, lift, drag, gravity and environmental wind.

All domains share `VehicleState`, `VehicleInput` and `VehicleEnvironment`, while domain-specific configuration controls the force model.

## Architecture

```text
Vehicle
├── VehicleState
├── VehicleInput
├── VehicleEnvironment
├── GroundVehicleConfig ──> step_ground
├── WaterVehicleConfig  ──> step_water
└── AirVehicleConfig    ──> step_air
             │
             ├── Physics / collision system
             ├── World / terrain / water
             ├── Weather / wind
             ├── Animation / presentation
             └── Network / authoritative state
```

The vehicle solver is simulation authority; renderer and audio consume its state.

## Ground vehicles

The ground model currently includes:

- normalized throttle
- braking force
- rolling resistance
- speed-dependent drag
- gravity
- maximum-speed limiting
- ground-plane clamping

Production extensions required for a complete vehicle model include wheel/tire contact patches, suspension, differential/gearing, tire friction curves, slip ratio/angle, ABS/traction control and terrain material interaction.

## Water vehicles

The water model currently includes:

- propulsion/thrust
- longitudinal drag
- lateral drag
- buoyancy based on waterline submergence
- wind-relative velocity
- gravity
- maximum-speed limiting

Production extensions include hull hydrostatics, displaced-volume integration, wave/FFT-ocean coupling, planing, propeller/waterjet models, rudder forces, spray/foam and wake generation.

## Air vehicles

The air model currently includes:

- thrust
- dynamic pressure
- wing area
- lift coefficient
- parasitic drag
- wind-relative airspeed
- gravity
- maximum-speed limiting

Production extensions include angle-of-attack curves, stall, induced drag, control-surface moments, angular dynamics, propulsion models, atmosphere-density variation and aerodynamic surfaces evaluated independently.

## Determinism and authority

The current APIs are deterministic for identical floating-point inputs and iteration order. They do not access external services. A-TownChain integration must remain an explicit state/ownership interface and must not make consensus depend on renderer, audio or GPU state.

## Verification

Unit tests cover acceleration, buoyancy and lift. CI/workflow execution is intentionally deferred according to the current project workflow.

## Production status

This is a **multi-domain vehicle dynamics foundation**, not a claim of a complete AAA vehicle solver. The documented extensions above are required before declaring the subsystem production-ready.

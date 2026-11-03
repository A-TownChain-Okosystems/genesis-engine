# GEN-VEH-001 — Vehicle AI

## Ownership
Vehicle-specific control semantics live in atc-genesis-vehicle-ai.
World collision, streaming, rendering, networking, and persistence remain in
their existing Genesis Engine modules.

## Determinism
Identical inputs produce identical outputs. The core uses no randomness,
wall-clock access, external services, or mutable global state.

## Existing-first
Repository inspection found no existing vehicle, traffic, or vehicle-agent
module. Generic atc-genesis-ai remains generic agent/navigation logic.

## Initial implementation
- VehicleSpec / VehicleState / VehicleCommand
- deterministic VehicleAgent
- waypoint steering
- speed limits
- obstacle-aware braking
- defensive/normal/sport profiles
- deterministic vehicle kinematics
- unit tests for bounds, acceleration, braking, reverse, obstacle response

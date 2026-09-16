# Genesis Engine Gameplay Systems

## Purpose

This document maps the genre-capability registry to concrete reusable gameplay foundations. The engine is genre-agnostic: games compose systems rather than inheriting a fixed genre implementation.

## Implemented foundations

### Movement
`GameplayRuntime` samples digital/analog input, clamps axes, normalizes diagonal movement and applies deterministic movement to ECS transforms.

### Interaction
`Interaction` represents an explicit source/target interaction with a typed action (`Use`, `Talk`, `PickUp`, `Open`, `Activate`, `Inspect`). It carries no renderer or network dependency.

### Abilities
`Ability` provides a deterministic Ready → Active → Cooldown lifecycle. Duration and cooldown are explicit and non-negative. This is a foundation for skills, powers, actions, tools and gameplay abilities.

### Combat
Combat provides:
- typed damage;
- health and death state;
- resistance mitigation;
- explicit attack range/cooldown data;
- deterministic cooldown ticking;
- explicit ally/neutral/enemy relation vocabulary.

Combat is simulation state. Animation, VFX, audio, UI and networking must consume combat events rather than become the authority for damage.

## Genre composition

The genre registry in `modules/atc-genesis-gameplay/src/genre.rs` remains the capability catalogue. These concrete systems are the first executable layer beneath that catalogue. Future systems should follow the same rule: reusable deterministic simulation first, presentation/integration second.

## Next implementation layers

1. Character controller and locomotion modes.
2. Navigation, agents, behavior trees, utility AI, squads and factions.
3. Inventory, equipment, stats, skills and crafting.
4. Quest, dialogue and narrative state.
5. Economy, trading, research, diplomacy and territory.
6. Building, farming, resources and destruction.
7. Traffic, racing, sports and advanced vehicle rules.
8. Flight/space simulation and vehicle damage.
9. Save/load, replay and deterministic rollback primitives.
10. Multiplayer replication, prediction, matchmaking and lobbies.
11. Procedural generation and world streaming.
12. Accessibility, localization, modding and scripting.

## Verification status

Unit tests were added for the new combat and interaction primitives. CI/workflow execution and merge are intentionally not performed in this implementation step. Production readiness remains unestablished until integration, performance, security, networking and platform-specific validation are completed.

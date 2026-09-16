# Genesis Engine — Genre Capability Matrix

## Ziel

Genesis Engine bleibt ein **general-purpose realtime engine**. Genres sind keine getrennten Engines und keine harten Runtime-Modi. Gemeinsame Systeme werden einmal implementiert und durch Spiel-spezifische Kombinationen aktiviert.

## Abgedeckte Genre-Familien

Action, Adventure, RPG/ARPG/MMORPG, Strategy/RTS/Tactics, Simulation/Management/City Builder, Survival/Horror/Stealth, FPS/TPS/Shooter, Racing/Sports/Fighting, Platformer/Metroidvania, Roguelike/Roguelite, Sandbox/Open World/Immersive Sim, Puzzle/Rhythm/Party, Card/Board/Turn-Based, Narrative/Visual Novel, MOBA/Battle Royale/Tower Defense, Flight Sim/Space Sim/Vehicle Sim sowie Educational/Serious Games und hybride Genres.

## Gemeinsame Engine-Komponenten

```text
Core / ECS / Jobs / Memory
├── World / Streaming / Terrain / Procedural Generation
├── Rendering / Materials / Lighting / Sky / Weather / Water
├── Physics / Vehicles / Destruction
├── Animation / Character Controller / Cinematics
├── Audio / Music / Spatial Audio
├── Input / Camera / UI / Accessibility / Localization
├── Gameplay / Interaction / Dialogue / Quest / Inventory
├── Combat / Ballistics / Stats / Skills
├── AI / Navigation / Behavior / Utility / Factions / Squads
├── Economy / Trading / Crafting / Building / Research / Diplomacy
├── Multiplayer / Replication / Prediction / Matchmaking / Replay
├── Save / Load / Replay / Photo Mode
├── Assets / Editor / SDK / Tools / Build / Packaging
└── Modding / Scripting / Diagnostics / Profiling
```

## Genre-spezifische Capability-Gruppen

- **Action/FPS/TPS:** Character Controller, Combat, Ballistics, Cover, AI, animation, camera, destruction.
- **RPG/ARPG/MMORPG:** Stats, equipment, inventory, quests, dialogue, skills, crafting, economy, factions, persistent multiplayer.
- **Strategy/RTS/Tactics:** squads, formations, resource management, research, territory, diplomacy, deterministic commands.
- **Simulation/Management/City Builder:** agents, schedules, economy, construction, resources, traffic, persistence, analytics.
- **Survival/Horror/Stealth:** needs, crafting, stealth perception, darkness, audio cues, AI, weather, procedural worlds.
- **Racing/Sports/Vehicle Sim:** vehicle dynamics, tracks/arenas, timing, replay, telemetry, collision/damage, network prediction.
- **Flight/Space:** 6-DOF movement, atmosphere/flight model, navigation, instruments, celestial/environment simulation.
- **Platformer/Metroidvania:** traversal controller, ledges, climbing, abilities, camera zones, encounter scripting.
- **Roguelike/Roguelite:** procedural generation, deterministic seeds, run state, meta progression, encounter director.
- **Puzzle/Card/Board/Turn-Based:** deterministic rules, state machines, turn manager, validation, replay and serialization.
- **Narrative/Visual Novel:** dialogue graphs, branching state, localization, cinematic sequencing, save checkpoints.
- **MOBA/Battle Royale/Tower Defense:** authoritative server simulation, prediction/replication, matchmaking, spatial queries, waves, objectives.
- **Sandbox/Open World/Immersive Sim:** systemic interaction, streaming, procedural generation, factions, AI, physics, modding.

## Implementation rule

The capability catalogue is a registry, not a claim that every listed feature is already production-complete. Each capability must progress through implementation, tests, profiling, security review and evidence before being marked production-ready.

The catalogue is exposed by `atc-genesis-gameplay::genre` so future gameplay modules can declare their required capabilities without creating genre-specific engine forks.

## Verification status

The current branch adds the genre catalogue and tests its presence/stability. CI and full integration verification remain deferred until the planned CI phase.

# Genesis Franchise Factory

The Franchise Factory is an integrated subsystem of `genesis-engine`; deterministic production contracts are now implemented canonically in Rust.

## Boundary

```text
Franchise Factory
  ├─ 17 AI production workflows
  ├─ Game Factory dependency graph
  ├─ Artifact + provenance contracts
  ├─ Franchise core pipeline
  ├─ DAO model
  └─ Lifecycle Manager
          │
          ▼
Genesis Engine Runtime / Build / Editor / SDK
          │
          ▼
Games and franchises
```

`genesis-chronicles` remains an independent consumer/flagship title and is not a dependency of this subsystem.

## Components

- `gff/core.py` — AD-20 compatibility/reference adapter (Rust is canonical)
- `gff/dao.py` — ATC-9900 franchise DAO model
- `gff/lifecycle.py` — AD-43 compatibility/reference adapter (Rust is canonical)
- `gff/spec_loader.py` — canonical `.atc` descriptor loader
- `gff/workflows.py` — provider-neutral workflow reference/adapter layer (Rust validation is canonical)
- `gff/artifacts.py` — typed artifact/provenance/evidence contracts
- `gff/game_factory.py` — deterministic Game Factory dependency graph

## Game Factory

The Game Factory models Concept, World, Lore, Character, Creature, Combat, Quest, Level, Item, Weapon, Animation, AI NPC, Audio, VFX, Economy, Multiplayer, Build, Testing and LiveOps as typed production stages.

The graph fails closed on missing producers and dependency cycles and exposes a deterministic topological order.

## AI boundary

The workflow engine does not call an AI provider directly. Model providers, credentials and external side effects are supplied through injected executors. This preserves deterministic orchestration and keeps provider concerns outside the core.

## Engine integration

The Rust implementation under `modules/atc-genesis-franchise-factory/` is the canonical production core. `franchise_factory/gff/` remains only as a compatibility/reference adapter until Rust↔Python conformance and integration evidence permit removal.

Production adapters to engine runtime systems, GCL/ATC-VM and external providers remain explicit integration boundaries and must be backed by implementation evidence before being marked production-ready.

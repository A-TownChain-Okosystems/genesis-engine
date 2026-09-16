# Gameplay Presets

Genesis Engine provides reusable data presets for common game-design configurations. A preset is a starting configuration; it does not force a game genre or prevent custom values.

## Preset families

| Family | Examples | Purpose |
|---|---|---|
| Inventory | RPG, Survival, MMO, Tactical | slots, hotbar capacity, stack limits |
| Character slots | Single, RPG Party, MMO Roster, Squad | roster, active character, party size |
| Skill tree | Simple, Classic, RPG, MMO | node count, branches, ranks, starting points |
| Ability bar | Action, RPG, MMO, Hero Shooter | active, ultimate and passive slots |
| Equipment | RPG, Survival, Tactical | weapon, armor and accessory slots |
| Loadout | RPG, Survival, Tactical | combined inventory + abilities + equipment defaults |

## Design rule

Presets are immutable constants and are intentionally independent from rendering, networking and A-TownChain. Games may copy the values into runtime configuration and modify them through their own configuration layer.

## Examples

```rust
use atc_genesis_gameplay::{InventoryPreset, SkillTreePreset, LoadoutPreset};

let inventory = InventoryPreset::RPG;
let skills = SkillTreePreset::RPG;
let loadout = LoadoutPreset::RPG;
```

## Extension path

Future preset families should cover character progression, quest templates, dialogue graphs, crafting recipes, economy models, AI archetypes, vehicle loadouts, racing rules, sports rules, multiplayer session profiles, UI layouts and accessibility profiles.

## Status

`IMPLEMENTED / FOUNDATION`

Unit tests exist for the preset module. Full editor integration, serialization/schema compatibility and runtime validation remain production-readiness work.

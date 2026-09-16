# Item & Weapon System

## Ziel

Genesis Engine provides deterministic, reusable item and weapon primitives that can support RPG, FPS, TPS, survival, tactical, racing, simulation and hybrid games without coupling gameplay to rendering or blockchain state.

## Item model

`ItemDefinition` describes static item data:

- stable ID
- item kind
- stack limit
- weight

`ItemStack` represents runtime quantity. `Inventory` provides deterministic stacking, counting and atomic-enough removal semantics: failed removal does not mutate the inventory.

## Weapon model

`WeaponDefinition` describes static weapon data:

- weapon kind
- damage type
- base damage
- range
- magazine size
- ammunition type
- durability

`WeaponInstance` contains runtime ammunition and durability. `WeaponLoadout` provides bounded equipment slots.

`WeaponInstance::fire()` consumes exactly one ammunition unit and one durability unit when both are available.

## Preset integration

The item/weapon primitives complement the existing gameplay presets:

- inventory presets define container capacity/layout policy
- equipment presets define slot policy
- loadout presets define starting configuration
- weapon definitions remain data-driven and are not hard-coded to a genre

## Safety and determinism

- empty IDs are rejected by inventory insertion
- zero stack limits are rejected
- inventory removal checks total availability before mutation
- weapon firing cannot consume ammunition or durability below zero
- loadout indices are bounds checked
- runtime state contains no renderer or blockchain dependency

## Not production-complete

The foundation does not yet implement projectile simulation, recoil, spread, hit validation, armor penetration, attachment systems, durability repair, crafting, loot tables, serialization compatibility, networking authority, or anti-cheat validation. Those belong to later specialized systems.

**Status: FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED**

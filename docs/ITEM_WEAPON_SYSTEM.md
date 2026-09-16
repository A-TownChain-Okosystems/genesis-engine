# Item & Weapon System

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

## Item layer

`ItemDefinition` defines stable item identity, category, stack limit and weight. `ItemStack` and `Inventory` provide deterministic quantity management. Failed removals do not mutate the inventory.

## Weapon layer

`WeaponDefinition` describes weapon kind, damage, damage type, range, magazine size, ammunition type and durability. `WeaponInstance` stores runtime ammunition/durability and `WeaponLoadout` manages bounded slots.

## Extended systems

### Armor
`ArmorStats` supports physical, fire, cold, electric and poison mitigation with resistance clamped to 0..100%.

### Ammunition
`Ammunition` carries damage-type and percentage modifier metadata. Ammunition remains data-driven and can later be bound to weapon compatibility rules.

### Attachments
`WeaponAttachment` provides deterministic additive modifiers for damage, range and magazine capacity. `WeaponAttachmentState` aggregates installed modifiers.

### Rarity
`Rarity` provides the common/uncommon/rare/epic/legendary classification primitive. It is metadata, not a hidden stat multiplier.

### Loot
`LootTable` uses weighted deterministic selection from an explicit seed. It does not use wall-clock randomness.

### Crafting
`Recipe` and `RecipeIngredient` provide deterministic ingredient validation and output creation. Missing ingredients leave inventory unchanged.

## Architectural boundaries

The system is independent of rendering, physics backends, UI and A-TownChain consensus. Network authority, persistence and anti-cheat validation must be implemented at the appropriate higher layers.

## Remaining production layers

Projectile/hit simulation, hitboxes, armor penetration, attachment compatibility, ammunition consumption rules, loot generation policies, affixes, crafting stations, serialization/versioning, authoritative multiplayer validation, replication and editor tooling remain separate work items.

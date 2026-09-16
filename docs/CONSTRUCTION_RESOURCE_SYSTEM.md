# Construction and Resource System

The construction layer connects technology progression, resource stock and world building.

## Resource model

`ResourceCost` supports deterministic costs for wood, stone, metal and energy. `ResourceStock` validates affordability and atomically consumes the required resources.

## Construction

`ConstructionDefinition` defines a structure's stable ID, dimensions, resource cost and optional technology requirement. `ConstructionSite` creates instances only when the required technology is researched, the position is valid and resources are available.

Initial structures:

- House
- Workshop

## Integration

The system is independent of rendering. Technology is the progression gate; resources are the economic input; construction creates world-state instances; furniture can subsequently occupy constructed interiors.

## Production gaps

Still required for production: spatial collision/overlap checks, grid or socket placement, build previews, construction stages/workers, resource gathering, inventories, power networks, structural integrity, ownership/permissions, persistence, replication and editor integration.

**Status: FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED**

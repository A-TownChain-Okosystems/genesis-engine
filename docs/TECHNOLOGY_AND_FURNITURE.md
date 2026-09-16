# Technology and Furniture System

## Technology

The technology foundation models a deterministic research tree independent of rendering and networking.

### Technology tiers

Primitive → Stone → Bronze → Iron → Industrial → Modern → Digital → Futuristic.

### Categories

Agriculture, Construction, Energy, Manufacturing, Computing, Transportation, Medicine, Communication, Defense and Household.

Each technology defines a stable ID, category, tier, research cost and prerequisite IDs. Research is rejected when prerequisites are missing or the technology has already been researched.

Initial definitions include Agriculture, Construction, Electricity and Computing.

## Furniture

Furniture is represented as data definitions plus placed runtime instances.

Supported furniture categories include chairs, tables, beds, storage, desks, shelves, cabinets, workbenches, kitchens, lighting, decoration and sanitary objects.

A furniture definition contains dimensions, comfort, storage capacity and an optional required technology. Instances contain a deterministic millimetre-space position, quarter-turn rotation and durability.

`FurnitureRoom` provides a bounded placement foundation.

Initial definitions include Chair, Table, Bed and Cabinet.

## Integration

Technology progression can gate construction and furniture. Furniture remains independent of the renderer and can later be consumed by Editor, World, Building, UI, NPC needs and simulation systems.

## Production gaps

Not yet implemented: resource costs, construction jobs, power networks, crafting recipes, collision/occupancy volumes, sockets, room semantics, furniture interaction, NPC utility/comfort simulation, serialization schemas, editor tooling, multiplayer authority and large-scale procedural settlement generation.

**Status: FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED**

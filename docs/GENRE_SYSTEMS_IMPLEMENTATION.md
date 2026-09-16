# Genre Systems Implementation

## Ziel
Die Genre-Capability-Matrix wird in wiederverwendbare, deterministische Engine-Systeme überführt. Die Gameplay-Simulation bleibt unabhängig von Renderer und A-TownChain.

## Implementierungsstand

| Bereich | Status | Implementierung |
|---|---|---|
| Character | FOUNDATION | Grounded-State, Jump-Gating, Stamina-Recovery |
| Combat | FOUNDATION | Health, Damage, Attacks, Teams |
| AI / Navigation | IMPLEMENTED FOUNDATION | Grid-Graph, deterministisches A*, Behavior Tree, Utility Selection |
| RPG / Progression | IMPLEMENTED FOUNDATION | Inventory, Equipment, Stats, SkillTree |
| Quest / Dialogue | IMPLEMENTED FOUNDATION | Quest-State/Objectives, Dialogue-Graph und validierte Kanten |
| Strategy | FOUNDATION | deterministische Priority-Queue |
| Economy | FOUNDATION | Overflow-/Overdraw-sichere Kontofunktionen |
| Building | FOUNDATION | deterministisches Grid mit Placement/Removal |
| Multiplayer | FOUNDATION | Input-Normalisierung, Tick/Sequence, deterministischer Replication-Buffer |
| Racing/Sports | FOUNDATION | Checkpoint-/Progress-State und Lap-Fortschritt |
| Flight | FOUNDATION | Thrust/Lift-basierte Fluggrundlage |
| Space | FOUNDATION | Double-Precision-Position/Velocity-Integration |
| Narrative | IMPLEMENTED FOUNDATION | typisierte Events plus Dialogue-Graph |
| Modding | FOUNDATION | IDs, Versionen, Dependencies, Permissions und deterministische Registry |

## Neue AI-Schicht

`ai.rs` stellt drei engine-neutrale Primitive bereit:

- `GridNode` / `GridBounds` für deterministische Navigation.
- `astar()` mit stabiler Tie-Break-Reihenfolge über Kosten und Koordinaten.
- `BehaviorNode` mit `Sequence`, `Selector` und `Leaf`.
- `choose_utility()` für deterministische Utility-Auswahl; gleiche Scores werden über die niedrigere ID aufgelöst.

Diese Schicht ist bewusst noch kein vollständiges Navmesh-, Perception-, Learning- oder Squad-System.

## Neue RPG-/Narrative-Schicht

`rpg.rs` stellt bereit:

- deterministisch sortierte `Inventory`-Stacks,
- `Equipment` mit stabiler Slot-Verwaltung,
- additive `Stat`-Berechnung,
- eindeutige `SkillTree`-Freischaltungen,
- `Quest` mit Locked/Active/Completed/Failed-State,
- `DialogueGraph` mit validierten Choice-Zielen.

## Architekturregeln

1. Gameplay-Simulation bleibt unabhängig von Rendering und Blockchain.
2. Netzwerkdaten werden normalisiert und deterministisch sortiert.
3. AI darf keine implizite globale Zeit oder nichtdeterministische Iteration benötigen.
4. Mod-Abhängigkeiten und Permissions müssen explizit im Manifest stehen.
5. A-TownChain-Integration erfolgt ausschließlich über eine explizite Adaptergrenze.
6. Foundations sind nicht automatisch produktionsreife Subsysteme.

## Nächste Tiefenebenen

1. Character: Locomotion-State-Machine, Beschleunigung, Slope Handling, Crouch, Swim, Climb und Abilities.
2. AI: Navmesh, räumliche Kostenfelder, Perception, Behavior-Memory, Squad/Faction Coordination und replizierbarer AI-State.
3. RPG: Stack-/Weight-Regeln, Equipment-Stat-Modifikatoren, Crafting-Rezepte, Skill-Prerequisites, Save/Load.
4. Economy: Ressourcenketten, Produktion/Verbrauch, Markt/Orderbook, Trading, Diplomatie und Territory.
5. Building: Snapping, Support Constraints, Construction Phases, Damage/Destruction und Farming.
6. Multiplayer: Authority Model, Snapshots, Interpolation, Client Prediction, Reconciliation, Rollback, Matchmaking und Replay.
7. Racing/Sports: Event-/Rule-System, Timing, Penalties, Scoring und deterministische Standings.
8. Flight/Space: 6-DOF-Attitude-Dynamics, Aerodynamik, Propulsion, Orbital Mechanics und Staging.
9. Narrative: Conditions/Variables, Save-State, Localization, Cinematics und graphbasierte Runtime-Ausführung.
10. Modding: Dependency-DAG, Capability Enforcement, API-Versionierung, Load Isolation und Sandbox-Grenze.

## Test-/Evidence-Grenze

Unit-Tests decken die neuen Foundations ab. Integrationstests, Replay-Determinismus, Serialisierungs-Kompatibilität, Netzwerkinteroperabilität, Performance-Profiling und plattformspezifische Validierung sind weiterhin erforderlich.

**Status: IMPLEMENTED_FOUNDATIONS / PRODUCTION_NOT_ESTABLISHED**

# Production Stations & Tool System

## Zweck

Die Gameplay-Schicht erhält eine konkrete Grundlage für Produktionsstationen und Werkzeugverschleiß. Das ergänzt die zeitbasierten Crafting-Jobs um die physische Produktionsinfrastruktur.

## Produktionsstationen

`ProductionStation` beschreibt eine Station mit stabiler ID, Stationstyp, Kapazität und Aktivierungszustand.

Implementierte Stationstypen: Workbench, Forge, Kitchen, Loom und Laboratory.

Die Stationsprüfung ist deterministisch und akzeptiert nur aktivierte Stationen mit positiver Kapazität.

## Werkzeugverschleiß

`ToolState` verwaltet Tool-ID, aktuelle Haltbarkeit, maximale Haltbarkeit, Nutzung, Bruchzustand und Zustand in Promille. Die Haltbarkeit kann nicht unter null fallen.

## Integration

Die Gameplay-API exportiert `ProductionStation`, `ProductionStationKind`, `ToolState` sowie `WORKBENCH`, `FORGE` und `KITCHEN`.

`Technologie → Ressourcen → Inventar → Produktionsstation → Rezept → CraftingJob → CraftingQueue → Werkzeugverschleiß → Ausgabe`

## Tests

Implementiert sind Stations-Gating, deterministischer Werkzeugverschleiß, Haltbarkeitsgrenzen, Bruchzustand und Validierung ungültiger Werkzeugdefinitionen.

## Noch fehlende Produktionssysteme

Stationseingangs-/Ausgangsinventare, reale Stationsbelegung, Werkzeuganforderungen pro Rezept, NPC-Arbeiter, Produktionsqualität und Skills, Produktionsketten, Persistenz, Multiplayer-Autorität/Replikation, Editor/UI und vollständige Wirtschafts-/Produktionssimulation.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

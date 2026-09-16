# Production Stations & Tool System

## Zweck

Die Gameplay-Schicht stellt Produktionsstationen mit deterministischer Prüfung sowie getrennten Eingangs- und Ausgangsinventaren bereit. Das erweitert die zeitbasierten Crafting-Jobs um reale Produktionsinfrastruktur.

## Produktionsstationen

`ProductionStation` enthält:

- stabile Stations-ID
- Stationstyp
- Kapazität
- Aktivierungszustand
- Input-Inventar
- Output-Inventar

Implementierte Stationstypen:

- Workbench
- Forge
- Kitchen
- Loom
- Laboratory

`accepts()` akzeptiert nur die korrekte ID einer aktivierten Station mit positiver Kapazität.

## Stationsinventare

Input und Output sind getrennt. Materialien können deterministisch in das Input-Inventar gelegt und wieder entnommen werden. Produzierte Gegenstände werden getrennt im Output-Inventar gespeichert.

Ungültige Operationen verändern den Bestand nicht und geben die nicht akzeptierte Menge zurück.

## Werkzeugverschleiß

`ToolState` verwaltet:

- Tool-ID
- aktuelle Haltbarkeit
- maximale Haltbarkeit
- Nutzung pro Crafting-Aktion
- Bruchzustand
- Zustand in Promille

Die Haltbarkeit kann nicht unter null fallen. Ein gebrochenes Werkzeug kann keine weitere Nutzung durchführen.

## Integration

Die Gameplay-API exportiert `ProductionStation`, `ProductionStationKind`, `ToolState` sowie `WORKBENCH`, `FORGE` und `KITCHEN`.

Die Produktionskette lautet:

`Technologie → Ressourcen → Inventar → Produktionsstation → Input → Rezept → CraftingJob → CraftingQueue → Werkzeugverschleiß → Output`

## Tests

Implementiert sind:

- Stations-Gating
- ungültige Stationsdefinitionen
- Input-/Output-Speicherung
- Input-/Output-Entnahme
- deaktivierte Stationen verweigern Lageroperationen
- deterministischer Werkzeugverschleiß
- Haltbarkeitsgrenzen
- Bruchzustand
- Validierung ungültiger Werkzeugdefinitionen

## Noch fehlende Produktionssysteme

- echte Stationsbelegung und parallele Kapazitätsverwaltung
- Werkzeuganforderungen pro Rezept
- atomare Rezeptausführung zwischen Input, Job und Output
- NPC-Arbeiter und Job-Zuweisung
- Produktionsqualität und Skills
- Produktionsketten mit Zwischenprodukten
- Persistenz und Wiederaufnahme
- Multiplayer-Autorität/Replikation
- Editor/UI-Visualisierung
- vollständige Wirtschafts-/Produktionssimulation

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

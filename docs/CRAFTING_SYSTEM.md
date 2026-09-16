# Crafting System

## Zweck

Das Crafting-System bildet die deterministische Transformation von Ressourcen/Items zu neuen Items ab. Es ergänzt die vorhandenen Inventar-, Technologie-, Ressourcen-, Gebäude- und Möbel-Systeme.

## Datenmodell

`CraftingRecipe` definiert:

- eindeutige Rezept-ID
- Eingabematerialien und Mengen
- Ausgabe-Item und Menge
- optionale Crafting-Station
- optionale Technologie-Voraussetzung
- Herstellungsdauer in Ticks

`CraftFailure` unterscheidet ungültige Rezepte, fehlende Zutaten, falsche Station und fehlende Technologie.

## Ablauf

Ein Craft-Vorgang wird zunächst vollständig validiert. Erst danach werden die Zutaten aus dem Inventar entfernt und das Ergebnis erzeugt. Dadurch wird verhindert, dass ein fehlgeschlagener Validierungsschritt teilweise Ressourcen verbraucht.

Die Implementierung nutzt ausschließlich deterministische Zustände und Integer-Arithmetik.

## Erste Rezepte

- `BASIC_TOOLS_RECIPE`
  - Holz + Stein
  - Technologie: `construction`
  - Ergebnis: `stone_tool`
- `WORKBENCH_RECIPE`
  - Holz + Stein
  - Technologie: `construction`
  - Ergebnis: `workbench`

## Integration

Die aktuelle Systemkette ist:

`Technologie → Ressourcen → Inventar → Crafting → Gebäude/Möbel → Bedürfnisse/NPC`

Crafting ist als generisches System gehalten. Konkrete Rezepte können später für Survival, RPG, MMO, Simulation, Strategie, Fahrzeuge, Industrie oder andere Genres ergänzt werden.

## Tests

Tests decken ab:

- erfolgreiche Herstellung
- atomaren Verbrauch bei Erfolg
- Technologie-Gate
- Station-Gate
- fehlende Zutaten ohne Inventar-Mutation

## Produktionsstatus

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

Noch offen:

- zeitbasierte Crafting-Jobs und Warteschlangen
- Unterbrechung/Fortsetzung
- Werkzeuge und Werkzeugverschleiß
- Materialqualität und Substitution
- Skill-/Proficiency-System
- Produktionsketten und Zwischenprodukte
- stationäre Produktionsanlagen
- Multiplayer-Autorität/Replikation
- Persistenz
- Editor- und Asset-Pipeline
- Balancing

CI/Workflow-Runs wurden für diese Änderung nicht ausgeführt.

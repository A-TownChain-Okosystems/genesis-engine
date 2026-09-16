# Atomic Production Execution

## Zweck

Dieses Modul verbindet Rezepte, Produktionsstationen, Inventare, zeitbasierte Crafting-Jobs und Werkzeuge zu einem deterministischen Produktionsablauf.

## Start einer Produktion

`start_production` führt alle Vorbedingungen vor der Mutation aus:

1. Rezeptstruktur validieren
2. Produktionsstation prüfen
3. Technologieanforderung prüfen
4. Zutatenbestand prüfen
5. Werkzeugzustand prüfen
6. `CraftingJob` erzeugen
7. Zutaten atomar aus dem Spielerinventar entfernen
8. Werkzeug genau einmal verbrauchen

Fehlgeschlagene Vorbedingungen verändern das Inventar nicht.

## Abschluss

`finish_production` akzeptiert nur vollständig abgearbeitete Jobs und legt das Ergebnis in das Output-Inventar der Produktionsstation. Eine deaktivierte oder ungültige Station blockiert die Ausgabe.

## Datenfluss

`Technologie → Ressourcen → Inventar → Station → Rezeptvalidierung → Input-Verbrauch → CraftingJob → Queue/Tick → Werkzeugverschleiß → Output`

## Fehlerklassen

- `InvalidRecipe`
- `MissingIngredient`
- `WrongStation`
- `MissingTechnology`
- `MissingTool`
- `ToolBroken`
- `OutputBlocked`

## Tests

Die Implementierung enthält Tests für:

- atomare Validierungsfehler
- Verbrauch der Eingaben
- zeitgesteuerten Abschluss
- Ausgabe in das Stationsinventar
- Werkzeugverbrauch

## Produktionsgrenzen

Noch nicht umgesetzt sind Worker-/NPC-Zuweisung, parallele Stationsbelegung anhand realer Kapazitäten, Persistenz/Wiederaufnahme, Multiplayer-Autorität, Qualitäts- und Skill-Systeme sowie eine vollständige Produktionsökonomie.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

# Atomic Production Execution

## Zweck

Dieses Modul verbindet Rezepte, Produktionsstationen, Inventare, zeitbasierte Crafting-Jobs und Werkzeuge zu einem deterministischen Produktionsablauf.

## Start einer Produktion

`start_production` validiert sämtliche Vorbedingungen vor einer dauerhaften Inventarmutation:

1. Rezeptstruktur validieren
2. Produktionsstation und Rezeptbindung prüfen
3. Technologieanforderung prüfen
4. Zutatenbestand prüfen
5. optionale Werkzeuganforderung und Werkzeugzustand prüfen
6. `CraftingJob` erzeugen
7. Werkzeug genau einmal verbrauchen
8. Zutaten atomar aus dem Spielerinventar entfernen

Bei Validierungsfehlern bleiben Inventar und Werkzeug unverändert. Die Stations-ID ist als statische ID ausgelegt, damit die vordefinierten Stationskonstanten typsicher bleiben.

## Abschluss

`finish_production` akzeptiert nur vollständig abgearbeitete Jobs und legt das Ergebnis in das Output-Inventar der Produktionsstation. Eine deaktivierte oder ungültige Station blockiert die Ausgabe.

## Datenfluss

`Technologie → Ressourcen → Inventar → Station → Rezeptvalidierung → Werkzeugprüfung → Input-Verbrauch → CraftingJob → Queue/Tick → Werkzeugverschleiß → Output`

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
- fehlende oder falsche Werkzeuge ohne Mutation
- Verbrauch der Eingaben
- zeitgesteuerten Abschluss
- Ausgabe in das Stationsinventar
- Werkzeugverschleiß

## Noch offene technische Punkte

`ProductionStation::capacity` validiert derzeit die Nutzbarkeit der Station, begrenzt aber noch nicht die Anzahl paralleler Jobs oder die Anzahl gespeicherter Inventarstacks. Diese Semantik muss vor einer Produktionsfreigabe explizit als Job-Kapazität oder Lagerkapazität modelliert werden.

Weitere offene Punkte sind Worker-/NPC-Zuweisung, Persistenz/Wiederaufnahme, Multiplayer-Autorität, Qualitäts- und Skill-Systeme sowie eine vollständige Produktionsökonomie.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

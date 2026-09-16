# Atomic Production Execution

## Zweck

Dieses Modul verbindet Rezepte, Produktionsstationen, Inventare, zeitbasierte Crafting-Jobs und Werkzeuge zu einem deterministischen Produktionsablauf.

## Start einer Produktion

`start_production` validiert sämtliche Vorbedingungen vor einer dauerhaften Inventarmutation:

1. Rezeptstruktur validieren
2. Produktionsstation und Rezeptbindung prüfen
3. freie Stationskapazität prüfen
4. Technologieanforderung prüfen
5. Zutatenbestand prüfen
6. optionale Werkzeuganforderung und Werkzeugzustand prüfen
7. `CraftingJob` erzeugen
8. Stationsslot reservieren
9. Werkzeug genau einmal verbrauchen
10. Zutaten aus dem Spielerinventar entfernen

Bei Validierungsfehlern bleiben Inventar, Werkzeug und Stationsbelegung unverändert. Schlägt eine Mutation nach der Reservierung fehl, wird die Stationsreservierung wieder freigegeben.

## Stationskapazität

`ProductionStation::capacity` ist jetzt explizit als **maximale Anzahl gleichzeitig aktiver Produktionsjobs** modelliert.

- `active_jobs` zählt reservierte Produktionsjobs.
- `reserve_job()` verweigert weitere Jobs bei voller Kapazität.
- `release_job()` gibt einen Slot deterministisch frei.
- Eine deaktivierte Station kann keine neuen Jobs reservieren.

Damit ist `capacity` nicht mit Lager-/Inventarkapazität zu verwechseln. Die Input-/Output-Inventare besitzen weiterhin eine separate, noch zu modellierende Lagerbegrenzung.

## Abschluss

`finish_production` akzeptiert nur vollständig abgearbeitete Jobs der korrekten Station. Nach erfolgreicher Output-Erzeugung wird der Stationsslot freigegeben.

Ein bereits abgeschlossener Job kann nicht erneut ausgegeben werden (`AlreadyCompleted`). Dadurch wird eine doppelte Output-Erzeugung verhindert.

Eine deaktivierte Station oder eine nicht passende Stations-ID blockiert die Ausgabe.

## Datenfluss

`Technologie → Ressourcen → Inventar → Station → Kapazitätsreservierung → Rezeptvalidierung → Werkzeugprüfung → Input-Verbrauch → CraftingJob → Queue/Tick → Werkzeugverschleiß → Output → Kapazitätsfreigabe`

## Fehlerklassen

- `InvalidRecipe`
- `MissingIngredient`
- `WrongStation`
- `StationCapacity`
- `MissingTechnology`
- `MissingTool`
- `ToolBroken`
- `OutputBlocked`
- `AlreadyCompleted`

## Tests

Die Implementierung enthält Tests für:

- atomare Validierungsfehler
- Stationskapazitätsgrenze
- fehlende oder falsche Werkzeuge ohne Mutation
- Verbrauch der Eingaben
- zeitgesteuerten Abschluss
- Ausgabe in das Stationsinventar
- Werkzeugverschleiß
- Freigabe der Stationskapazität
- Schutz gegen doppelte Output-Erzeugung

## Noch offene technische Punkte

Die Job-Kapazität ist implementiert. Noch nicht umgesetzt ist eine separate Begrenzung der Anzahl gespeicherter Input-/Output-Inventarstacks.

Weitere offene Punkte sind Worker-/NPC-Zuweisung, Persistenz/Wiederaufnahme, Multiplayer-Autorität, Qualitäts- und Skill-Systeme sowie eine vollständige Produktionsökonomie.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

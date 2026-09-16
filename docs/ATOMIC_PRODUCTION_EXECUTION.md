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
7. Input-Lagerkapazität prüfen
8. `CraftingJob` erzeugen
9. Stationsslot reservieren
10. Werkzeug genau einmal verbrauchen
11. Zutaten aus dem Spielerinventar entfernen

Bei Validierungsfehlern bleiben Inventar, Werkzeug und Stationsbelegung unverändert. Schlägt eine Mutation nach der Reservierung fehl, wird die Stationsreservierung wieder freigegeben.

## Stationskapazität

`ProductionStation::capacity` ist die maximale Anzahl gleichzeitig aktiver Produktionsjobs.

- `active_jobs` zählt reservierte Produktionsjobs.
- `reserve_job()` verweigert weitere Jobs bei voller Kapazität.
- `release_job()` gibt einen Slot deterministisch frei.
- Eine deaktivierte Station kann keine neuen Jobs reservieren.

## Lagerkapazität

Input und Output besitzen getrennte Stack-Limits:

- `input_slots`
- `output_slots`

`input_has_storage()` und `output_has_storage()` prüfen vorhandene freie Stackplätze oder noch nicht volle bestehende Stacks. Ungültige Einlagerungen werden vor der Mutation abgewiesen.

Damit sind **Job-Kapazität** und **Lagerkapazität** getrennte Ressourcen und werden nicht mehr über dasselbe Feld semantisch vermischt.

## Abschluss

`finish_production` akzeptiert nur vollständig abgearbeitete Jobs der korrekten Station. Vor der Output-Mutation wird die verfügbare Output-Lagerkapazität geprüft. Nach erfolgreicher Output-Erzeugung wird der Stationsslot freigegeben.

Ein bereits abgeschlossener Job kann nicht erneut ausgegeben werden (`AlreadyCompleted`). Dadurch wird doppelte Output-Erzeugung verhindert.

Wenn der Output-Speicher voll ist, bleiben Job und Stationsreservierung erhalten; der Abschluss kann nach Freigabe von Lagerplatz erneut versucht werden.

## Datenfluss

`Technologie → Ressourcen → Inventar → Station → Job-Kapazität → Rezeptvalidierung → Lagerprüfung → Werkzeugprüfung → Input-Verbrauch → CraftingJob → Queue/Tick → Werkzeugverschleiß → Output-Lagerprüfung → Output → Kapazitätsfreigabe`

## Fehlerklassen

- `InvalidRecipe`
- `MissingIngredient`
- `WrongStation`
- `StationCapacity`
- `MissingTechnology`
- `MissingTool`
- `ToolBroken`
- `InputStorageBlocked`
- `OutputBlocked`
- `AlreadyCompleted`

## Tests

Die Implementierung enthält Tests für:

- atomare Validierungsfehler
- Stationskapazitätsgrenze
- Input-/Output-Stackgrenzen
- vorhandene teilgefüllte Stacks
- fehlende oder falsche Werkzeuge ohne Mutation
- Verbrauch der Eingaben
- zeitgesteuerten Abschluss
- Output-Erzeugung
- Werkzeugverschleiß
- Freigabe der Stationskapazität
- Schutz gegen doppelte Output-Erzeugung
- blockierten Output ohne Freigabe des laufenden Jobs

## Noch offene technische Punkte

Die grundlegenden Job- und Lagergrenzen sind implementiert. Noch offen sind Worker-/NPC-Zuweisung, Persistenz/Wiederaufnahme, Multiplayer-Autorität, Qualitäts- und Skill-Systeme sowie eine vollständige Produktionsökonomie.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

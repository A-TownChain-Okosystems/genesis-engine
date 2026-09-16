# Atomic Production Execution

## Zweck

Dieses Modul verbindet Rezepte, Produktionsstationen, Inventare, zeitbasierte Crafting-Jobs, Werkzeuge und optionale Worker/NPC-Zuweisungen zu einem deterministischen Produktionsablauf.

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

`input_has_storage()` und `output_has_storage()` prüfen vorhandene freie Stackplätze oder noch nicht volle bestehende Stacks.

## Worker-/NPC-Zuweisung

`ProductionWorker` modelliert eine optionale Produktionsarbeitskraft:

- stabile Worker-ID
- `skill_permille` im Bereich `0..=1000`
- exklusive Belegung über `available`
- Zuordnung zur Stations-ID
- deterministisches Freigeben
- deterministische Dauerberechnung über `effective_duration()`

`ProductionJob::worker_id` bleibt optional, sodass Jobs weiterhin ohne Worker gestartet werden können. `assign_worker()` verhindert Doppelzuweisung; `release_worker()` löst die Zuordnung wieder.

Die Skill-Wirkung ist jetzt tatsächlich in der Jobdauer materialisiert: Skill `0` behält die Basisdauer, Skill `1000` reduziert sie deterministisch auf 50 %, mit mindestens einem Tick. Die Anpassung erfolgt beim Worker-Assignment und setzt `total_ticks` und `remaining_ticks` gemeinsam, bevor der Job weiter tickt.

## Abschluss

`finish_production` akzeptiert nur vollständig abgearbeitete Jobs der korrekten Station. Vor der Output-Mutation wird die verfügbare Output-Lagerkapazität geprüft. Nach erfolgreicher Output-Erzeugung wird der Stationsslot freigegeben.

Ein bereits abgeschlossener Job kann nicht erneut ausgegeben werden (`AlreadyCompleted`). Wenn der Output-Speicher voll ist, bleiben Job und Stationsreservierung erhalten.

## Datenfluss

`Technologie → Ressourcen → Inventar → Station → Job-Kapazität → Rezeptvalidierung → Lagerprüfung → Werkzeugprüfung → Input-Verbrauch → CraftingJob → Worker-Zuweisung → Skill-Anpassung → Queue/Tick → Output-Lagerprüfung → Output → Kapazitätsfreigabe → Worker-Freigabe`

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
- exklusive Worker-Zuweisung
- Worker-Freigabe
- deterministische Skill-Dauer und tatsächliche Jobdauer-Anpassung
- ungültige Worker-Konfiguration

## Noch offene technische Punkte

Worker-/NPC-Zuweisung und die deterministische Skill-Wirkung sind implementiert. Noch offen sind Worker-Persistenz/Wiederaufnahme, Abwesenheit/Unterbrechung, Multiplayer-Autorität, Qualitäts- und weiterführende Skill-Systeme sowie eine vollständige Produktionsökonomie.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

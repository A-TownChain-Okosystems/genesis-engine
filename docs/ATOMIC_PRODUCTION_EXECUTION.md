# Atomic Production Execution

## Zweck

Dieses Modul verbindet Rezepte, Produktionsstationen, Inventare, zeitbasierte Crafting-Jobs, Werkzeuge, Worker/NPC-Zuweisungen und persistierbare Resume-Zustände zu einem deterministischen Produktionsablauf.

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
- deterministische Dauerberechnung

`assign_worker()` macht die Skill-Auswirkung autoritativ für den Job: `total_ticks` und `remaining_ticks` werden beim ersten Worker-Assignment auf die effektive Dauer gesetzt. Doppelzuweisung ist verboten.

`release_worker()` entfernt die Job-Zuordnung und macht den Worker wieder verfügbar. Die Freigabe muss nach erfolgreichem Produktionsabschluss durch den Aufrufer erfolgen; `finish_production` bleibt bewusst Worker-agnostisch.

## Persistenz und Wiederaufnahme

`ProductionJobState` ist ein deterministischer Snapshot eines laufenden Produktionsjobs. Gespeichert werden:

- Rezept-ID
- Stations-ID
- verbleibende und gesamte Ticks
- Pausezustand
- Output-ID und Menge
- Werkzeug-ID
- Worker-ID
- Output-Commit-Status

`ProductionJobState::capture()` erzeugt einen Snapshot.

`restore_production_job()` stellt einen bereits identifizierten Job wieder her. Vor der Mutation werden Zustand, Rezept-ID, Stations-ID und Output-Identität validiert. Ein ungültiger oder nicht passender Snapshot verändert den Zieljob nicht.

Die Wiederaufnahme rekonstruiert absichtlich keinen neuen Stationsslot und konsumiert keine Ressourcen erneut. Persistenz ist damit eine Zustandswiederherstellung eines bereits gestarteten Jobs, keine zweite Job-Erzeugung.

## Pause und Resume

Der Pausezustand wird persistiert. Ein pausierter `CraftingJob` verarbeitet keine Ticks. Nach erfolgreicher Wiederherstellung kann der Job über die vorhandenen `resume()`-/`tick()`-Operationen fortgesetzt werden.

## Abschluss

`finish_production` akzeptiert nur vollständig abgearbeitete Jobs der korrekten Station. Vor der Output-Mutation wird die verfügbare Output-Lagerkapazität geprüft. Nach erfolgreicher Output-Erzeugung wird der Stationsslot freigegeben.

Ein bereits abgeschlossener Job kann nicht erneut ausgegeben werden (`AlreadyCompleted`). Wenn der Output-Speicher voll ist, bleiben Job und Stationsreservierung erhalten.

## Datenfluss

`Technologie → Ressourcen → Inventar → Station → Job-Kapazität → Rezeptvalidierung → Lagerprüfung → Werkzeugprüfung → Input-Verbrauch → CraftingJob → Worker-Zuweisung/Skill → Queue/Tick → Snapshot → Restore → Output-Lagerprüfung → Output → Stationsfreigabe → Worker-Freigabe`

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
- `ProductionPersistenceFailure::InvalidState`
- `ProductionPersistenceFailure::RecipeMismatch`
- `ProductionPersistenceFailure::StationMismatch`
- `ProductionPersistenceFailure::OutputMismatch`
- `ProductionPersistenceFailure::AlreadyCompleted`

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
- exklusive Worker-Zuweisung
- Worker-Freigabe
- Skill-basierte autoritative Jobdauer
- Mindestdauer von einem Tick
- ungültige Worker-Konfiguration
- Snapshot-Erzeugung
- Wiederaufnahme mit erhaltenem Fortschritt
- Wiederaufnahme eines pausierten Jobs
- Ablehnung inkonsistenter Persistenzdaten ohne Mutation
- Ablehnung von Rezept-/Identitätsabweichungen

## Noch offene technische Punkte

Die deterministische Persistenz-/Resume-Foundation ist implementiert. Noch offen sind ein konkretes dauerhaftes Dateiformat bzw. Serializer, Crash-Atomicity des externen Speichers, Worker-Wiederzuordnung nach Prozessneustart, Abwesenheit/Unterbrechung, Multiplayer-Autorität, Qualitäts- und fortgeschrittene Skill-Systeme sowie eine vollständige Produktionsökonomie.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

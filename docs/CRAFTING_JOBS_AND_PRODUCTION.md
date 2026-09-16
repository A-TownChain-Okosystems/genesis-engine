# Crafting Jobs & Production Queue

## Zweck

Das bisherige Crafting-Modell beschreibt Rezeptvalidierung und sofortige Ausgabe. Diese Erweiterung führt eine deterministische zeitbasierte Produktionsgrundlage ein.

## CraftingJob

Ein Job enthält Rezept-ID, Gesamtzeit, verbleibende Zeit und Pausezustand. `tick()` reduziert die verbleibende Zeit saturierend. `pause()` und `resume()` ermöglichen Unterbrechungen ohne Zeitverlust.

## Fortschritt

`progress_permille()` liefert einen deterministischen Fortschrittswert von 0 bis 1000. Ein abgeschlossener Job besitzt `remaining_ticks == 0`.

## CraftingQueue

Die Queue verwaltet mehrere Jobs, schreibt sie um eine Tick-Anzahl fort und kann abgeschlossene Jobs gesammelt entfernen.

## Integration

Die Gameplay-API exportiert `CraftingJob` und `CraftingQueue`.

`Technologie → Ressourcen → Inventar → Rezeptvalidierung → Produktionsjob → Queue → Ausgabe`

## Tests

Implementiert sind Tests für deterministischen Fortschritt, Pause/Resume, Abschluss nach Ablauf der Dauer und das Entfernen abgeschlossener Jobs.

## Noch fehlende Produktionssysteme

Produktionsstationen und Besitz, Eingangs-/Ausgangsinventare, Werkzeuganforderungen und Verschleiß, Arbeiter/NPC-Zuweisung, Ressourcenunterbrechungen, Qualität und Skills, Produktionsketten, Persistenz, Multiplayer-Autorität/Replikation, Editor-Visualisierung und Wirtschaftssimulation.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

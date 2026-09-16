# Needs & Settlement System

## Zweck

Das Gameplay-Modul erhält eine deterministische Grundlage für menschliche Charaktere und NPCs. Bedürfnisse bilden die Brücke zwischen Welt, Gebäuden, Möbeln, Nahrung, Wasser, Schlaf und späterer Siedlungs-/Wirtschaftssimulation.

## Bedürfnisse

`Needs` führt fünf normierte Zustände im Bereich `0..=1000`:

- `hunger` — steigender Wert bedeutet zunehmenden Hunger
- `thirst` — steigender Wert bedeutet zunehmenden Durst
- `sleep` — steigender Wert bedeutet zunehmenden Schlafbedarf
- `comfort` — sinkender Wert bedeutet abnehmenden Wohnkomfort
- `health` — verbleibende Gesundheit

`tick()` ist deterministisch. Hunger, Durst und Schlafbedarf steigen; Komfort sinkt. Bei kritischem Hunger oder Durst wird Gesundheit reduziert.

## Aktionen

`next_need_action()` bestimmt eine einfache deterministische Priorität:

1. Durst
2. Hunger
3. Schlaf
4. Komfort
5. Idle

Damit kann später ein NPC-Planer konkrete Weltaktionen wie Wasser holen, Nahrung konsumieren, schlafen oder Möbel aufsuchen ausführen.

## Integration

Das Modul ist direkt über `atc-genesis-gameplay` exportiert. Es kann mit den bereits implementierten Systemen verbunden werden:

`Technologie → Ressourcen → Gebäude → Möbel → Bedürfnisse → NPC-Aktionen`

Beispiele:

- Bett reduziert Schlafbedarf.
- Küche bzw. Nahrung reduziert Hunger.
- Wasserquelle reduziert Durst.
- Möbel erhöhen Komfort.
- Gebäude definieren räumliche Aufenthaltsmöglichkeiten.

## Determinismus

Die aktuelle Implementierung verwendet ausschließlich Integer-Arithmetik und deterministische Zustandsübergänge. Es gibt keine Zeit-, Zufalls- oder Plattformabhängigkeit.

## Tests

Enthalten sind Tests für:

- deterministischen Bedarfsausbau und Gesundheitsdruck
- Aktionspriorität
- begrenzte Erholung

CI/Workflow-Ausführung ist für diese Änderung noch nicht erfolgt.

## Produktionsstatus

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`

Noch offen sind insbesondere:

- tatsächliche Nahrung/Wasser-Inventare
- Bett-/Möbel-Interaktionen
- Pfadplanung und Job-System
- NPC-Utility-/GOAP-/Behavior-Integration
- Wohnraum- und Arbeitsplatzzuordnung
- Energie- und Versorgungssystem
- Persistenz und Netzwerkautorität
- Multiplayer-Replikation
- Editor-Unterstützung
- Balancing und Telemetrie

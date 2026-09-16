# Animal System

## Zweck

Das Animal System stellt eine deterministische Gameplay-Grundlage für Wildtiere, Nutztiere, Ökosysteme, Survival, Farming, Simulation und genreübergreifende Welten bereit.

## Datenmodell

`AnimalDefinition` beschreibt statische Speziesdaten:

- stabile Spezies-ID
- Tierkategorie
- maximale Gesundheit
- Bewegungsgeschwindigkeit
- Hungerverbrauch pro Tick
- Reproduktions-Cooldown

`AnimalInstance` enthält den Laufzeitstatus:

- Spezies
- Verhalten/State
- Gesundheit
- Hunger
- Alter
- Reproduktions-Timer

## Zustände

- Idle
- Roaming
- Grazing
- Hunting
- Fleeing
- Sleeping
- Following
- Dead

Die aktuelle Foundation implementiert insbesondere Hunger, Nahrung, Bedrohung, Flucht, Bewegungsauswahl und Reproduktion. Weitere Zustände sind für spätere spezialisierte Verhalten-Systeme vorgesehen.

## Umwelt

`AnimalEnvironment` stellt deterministische Eingaben bereit:

- Nahrung
- Wasser
- Bedrohungswert
- Temperatur

Damit kann das System später an Pflanzen, Wetter, Wasser, Terrain und World Streaming angebunden werden, ohne diese Systeme fest zu koppeln.

## Reproduktion

Zwei lebende Tiere derselben Spezies können bei ausreichender Gesundheit und abgelaufenem Reproduktions-Cooldown ein neues Tier erzeugen. Die Eltern erhalten anschließend denselben Cooldown.

## Herd/Flocken

`Herd` verwaltet Tiere mit einer festen Kapazität. Das verhindert unkontrolliertes Wachstum auf dieser Systemebene.

## Beispiel-Spezies

- Deer — Herbivore
- Wolf — Carnivore
- Cow — Livestock

Weitere Spezies sollen datengetrieben ergänzt werden.

## Determinismus

Die Simulation verwendet keine zeitabhängigen Zufallsquellen. Zustandsänderungen werden ausschließlich aus Definition, aktuellem Tierzustand und expliziter Umgebung berechnet.

## Tests

Enthalten sind Tests für:

- Bedrohung → Flucht
- Hunger → Grazing
- deterministische Reproduktion
- Herd-Kapazität

## Produktionsgrenzen

Noch nicht implementiert sind unter anderem:

- räumliche Navigation/Pathfinding für Tiere
- Herdenformationen und Flocking
- vollständige Predator/Prey-Simulation
- Fortpflanzungsgenetik
- Lebensphasen/Alterssimulation
- Krankheiten und Verletzungssysteme
- Animation/Animation State Machines
- Audio/Animal Vocalization
- LOD und Population Scaling
- persistente Save-/Load-Serialisierung
- autoritative Multiplayer-Replikation
- komplexe Ökosystem- und Nahrungskettenmodelle

**Status: FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED**

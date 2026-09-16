# Plant System

## Zweck

Das Genesis-Engine-Pflanzensystem bildet eine deterministische Grundlage für Landwirtschaft, Survival, Open World, Simulation, Crafting und prozedurale Ökosysteme.

## Datenmodell

`PlantDefinition` beschreibt eine Pflanzenart. `PlantInstance` enthält den Laufzeitstatus. `PlantEnvironment` beschreibt Wasser, Licht und Temperatur. `CropPlot` begrenzt die Zahl gleichzeitig gepflanzter Instanzen.

### Wachstumsphasen

`Seed -> Sprout -> Vegetative -> Flowering -> Fruiting -> Mature -> Withered`

Wachstum erfolgt nur bei ausreichender Wasserversorgung und innerhalb des definierten Lichtbereichs. Bei ungünstigen Bedingungen sinkt die Gesundheit. Bei vollständigem Gesundheitsverlust wird die Pflanze `Withered`.

## Vorlagen

Enthalten sind:

- `WHEAT`
- `CORN`
- `HERB`

Weitere Pflanzenarten können als reine `PlantDefinition` ergänzt werden, ohne die Runtime zu verändern.

## Ernte

Nur reife Pflanzen können geerntet werden. Die Erntemenge basiert deterministisch auf der Definition und einem optionalen Yield-Bonus. Nach der Ernte beginnt die Instanz wieder bei `Seed`.

## Integration

Das System ist unabhängig von Renderer, Netzwerk und A-TownChain-State. Die erzeugten Ernte-Items können über das vorhandene Item-/Inventory-System weiterverarbeitet werden.

## Produktionslücken

Noch nicht enthalten sind Bodenchemie, Nährstoffkreisläufe, Jahreszeiten, genetische Varianten, Krankheiten/Schädlinge, Bestäubung, Wachstum durch Temperaturmodelle, 3D-Wurzelsysteme, Landwirtschaftsgeräte, prozedurale Vegetationsverteilung, persistente Serialization und autoritative Multiplayer-Replikation.

**Status: FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED**

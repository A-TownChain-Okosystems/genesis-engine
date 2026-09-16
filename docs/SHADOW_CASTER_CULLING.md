# Shadow-Caster-Culling

## Ziel

Shadow-Pässe sollen nur relevante Geometrie an den Depth-Pass übergeben. Die Auswahl muss deterministisch und konservativ sein.

## Implementierung

`shadow_culling.rs` führt für Directional-Light-Cascades eine konservative Bounding-Sphere-Prüfung durch.

Der Ablauf ist:

1. Kamera und Cascade validieren.
2. Directional-Shadow-Projektion validieren.
3. Kamera-Basis normalisieren.
4. Für jeden Caster Tiefe, horizontale und vertikale Entfernung bestimmen.
5. Bounding-Sphere-Radius als Sicherheitsmarge verwenden.
6. Nicht relevante Caster verwerfen.
7. Entity-IDs sortieren und Duplikate entfernen.

Dadurch entstehen stabile Caster-Listen unabhängig von der Eingabereihenfolge.

## Fehlerbehandlung

Ungültige Kamera-, Cascade- oder Caster-Daten führen zu einem Fehler statt zu einer stillen Aufnahme potenziell ungültiger Geometrie.

## Architektur

Das Culling ist backend-neutral. Es erzeugt keine GPU-Kommandos und allokiert keine GPU-Ressourcen. Die resultierende `ShadowCasterSet` ist für spätere Vulkan-, D3D12- und Metal-Depth-Pässe vorgesehen.

## Tests

Abgedeckt sind:

- deterministische und sortierte Caster-Ausgabe
- Ausschluss außerhalb des Cascade-Tiefenbereichs
- fail-closed Verhalten bei ungültigen Bounds
- Shadow-Map-Auflösungsvalidierung

## Noch offen

- Culling gegen die vollständige Light-Space-Cascade statt gegen die konservative Camera-Space-Näherung
- BVH-beschleunigte Kandidatensuche
- GPU-driven/indirect Culling
- Point-Light-Cube-Face-Culling
- Spot-Light-Frustum-Culling
- tatsächlicher GPU-Depth-Pass

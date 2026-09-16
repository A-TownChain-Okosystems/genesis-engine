# Lighting and Shadows

## Ziel

Der Genesis Renderer besitzt eine deterministische Lighting- und Shadow-Pipeline-Foundation. Die Lichtregistrierung unterstützt Directional, Point und Spot Lights und validiert die relevanten Parameter. Lichtquellen werden nach `LightId` deterministisch geordnet.

## Beleuchtung

Unterstützte Lichttypen:

- Directional Light
- Point Light
- Spot Light

Die Lichtregistrierung validiert IDs, Richtungen, Intensität, Reichweite, Finite-Werte und Spot-Winkel.

## Schattenkonfiguration

`ShadowConfig` definiert:

- `ShadowMode::None`
- `ShadowMode::Hard`
- `ShadowMode::Pcf`
- 256/512/1024/2048/4096 Shadow-Map-Auflösung
- Depth Bias
- Normal Bias
- maximale Schattendistanz
- 1–8 Cascades

## Cascaded Shadow Maps

`directional_cascades()` erzeugt deterministische Splits aus logarithmischem und uniformem Anteil. Die letzte Cascade wird auf die tatsächlich zulässige maximale Distanz begrenzt.

## Shadow Pass Planning

`ShadowFramePlan` übersetzt die Lichtszene in backend-neutrale Shadow-Pass-Beschreibungen:

- Directional Light → ein Pass pro Cascade
- Point Light → Point-Cube-Projektion
- Spot Light → Perspective-Projektion

Jeder Pass besitzt eine stabile `ShadowPassId` aus Licht-ID und Slice. Die Planungsreihenfolge ist deterministisch.

## Render-Graph-Integration

`ShadowFramePlan::populate_render_graph()` verbindet die Shadow-Pässe mit dem bestehenden `RenderPassGraph`:

1. Jeder Shadow-Pass schreibt eine eigene Shadow-Resource.
2. Der Lighting-Pass liest alle erzeugten Shadow-Resources.
3. Der Lighting-Pass erhält explizite Abhängigkeiten auf alle Shadow-Pässe.
4. `RenderPassGraph::ordered_passes()` erzwingt dadurch `Shadow → Lighting`.
5. Shadow-Pass- und Resource-IDs werden deterministisch aus `LightId` und Slice abgeleitet.
6. Nicht darstellbare `LightId`-Werte werden fail-closed mit `LightIdTooLarge` abgewiesen.

## Shadow-Projektionsmathematik

`shadow_math.rs` implementiert nun backend-neutrale Directional-Light-Projektionsmatrizen:

- validiert Kamera-Projektion und Kamera-Basis
- validiert Cascade-Nah-/Fernbereich
- normalisiert die Licht-Richtung
- berechnet einen stabilen Frustum-Mittelpunkt für die Cascade
- bestimmt eine orthografische Shadow-Projektion
- erzeugt eine deterministische Light-View-Matrix
- kombiniert View und Orthographic Projection
- weist nicht-finite Ergebnisse zurück

Die Matrizen sind noch nicht an konkrete GPU-Clip-Space-Konventionen eines einzelnen Backends gebunden. Texel-Snapping und vollständige CSM-Stabilisierung folgen als eigene Stufe.

## Architekturgrenze

Die Shadow-Schicht erzeugt keine Vulkan-, D3D12-, Metal- oder OpenGL-Kommandos und reserviert keine GPU-Ressourcen. Die vorhandene Backend-Abstraktion bleibt für API-spezifische Ausführung zuständig.

## Datenfluss

`LightRegistry → ShadowConfig → Cascade/Projection Selection → ShadowFramePlan → Shadow Projection Math → RenderPassGraph (Shadow → Lighting) → Backend Shadow Resources → Shadow Depth Pass → Lighting Shader`

## Fehlerklassen

- `InvalidBias`
- `InvalidDistance`
- `InvalidCascadeCount`
- `UnsupportedLight`
- `LightIdTooLarge`
- `ShadowGraphError::Graph(...)`
- `ShadowMathError::InvalidCamera`
- `ShadowMathError::InvalidLightDirection`
- `ShadowMathError::NonFinite`

## Tests

Implementiert sind Tests für:

- Lichtvalidierung
- deterministische Lichtsortierung
- Cascade-Grenzen
- maximale Schattendistanz
- ungültige Shadow-Konfiguration
- deterministische Shadow-Light-Auswahl
- Directional-Light-Cascade-Passplanung
- stabile Pass-Reihenfolge
- `Shadow → Lighting` Render-Graph-Abhängigkeit
- fail-closed Behandlung nicht darstellbarer Licht-IDs
- deterministische Shadow-Matrizen
- Null-Lichtrichtung
- ungültige Cascade-Matrixbereiche

## Noch offene GPU-Stufen

1. echte Depth-Texture-Allokation
2. Shadow-Framebuffer/Render-Target-Aufbau
3. Texel-Snapping und CSM-Stabilisierung
4. Point-Light-Cubemap-Rendering
5. Spot-Light-Shadow-Maps
6. Shadow-Caster-Culling pro Pass
7. PCF/PCSS-Filterung in den Lighting-Shadern
8. Kontakt-/Self-Shadowing
9. Shadow-Caching und Update-Frequenz
10. Vulkan-Ausführung
11. Direct3D 12-Ausführung
12. Metal-Ausführung
13. GPU-Profiling und Speicherbudgetierung

## Status

`LIGHTING_FOUNDATION_IMPLEMENTED`

`SHADOW_PASS_PLANNING_IMPLEMENTED`

`RENDER_GRAPH_SHADOW_TO_LIGHTING_IMPLEMENTED`

`DIRECTIONAL_SHADOW_PROJECTION_MATH_IMPLEMENTED`

`GPU_SHADOW_RENDERING_NOT_ESTABLISHED`

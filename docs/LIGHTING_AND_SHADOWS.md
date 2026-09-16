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

Damit ist die Render-Graph-Abhängigkeit implementiert, ohne bereits eine konkrete GPU-API vorzutäuschen.

## Architekturgrenze

Die Shadow-Schicht erzeugt keine Vulkan-, D3D12-, Metal- oder OpenGL-Kommandos und reserviert keine GPU-Ressourcen. Die vorhandene Backend-Abstraktion bleibt für API-spezifische Ausführung zuständig.

## Datenfluss

`LightRegistry → ShadowConfig → Cascade/Projection Selection → ShadowFramePlan → RenderPassGraph (Shadow → Lighting) → Backend Shadow Resources → Shadow Depth Pass → Lighting Shader`

Die Render-Graph-Planung ist implementiert. Die tatsächliche GPU-Ausführung bleibt offen.

## Fehlerklassen

- `InvalidBias`
- `InvalidDistance`
- `InvalidCascadeCount`
- `UnsupportedLight`
- `LightIdTooLarge`
- `ShadowGraphError::Graph(...)`

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

## Noch offene GPU-Stufen

1. echte Depth-Texture-Allokation
2. Shadow-Framebuffer/Render-Target-Aufbau
3. Directional-Cascade View/Projection-Matrizen
4. Texel-Snapping und CSM-Stabilisierung
5. Point-Light-Cubemap-Rendering
6. Spot-Light-Shadow-Maps
7. Shadow-Caster-Culling pro Pass
8. PCF/PCSS-Filterung in den Lighting-Shadern
9. Kontakt-/Self-Shadowing
10. Shadow-Caching und Update-Frequenz
11. Vulkan-Ausführung
12. Direct3D 12-Ausführung
13. Metal-Ausführung
14. GPU-Profiling und Speicherbudgetierung

## Status

`LIGHTING_FOUNDATION_IMPLEMENTED`

`SHADOW_PASS_PLANNING_IMPLEMENTED`

`RENDER_GRAPH_SHADOW_TO_LIGHTING_IMPLEMENTED`

`GPU_SHADOW_RENDERING_NOT_ESTABLISHED`

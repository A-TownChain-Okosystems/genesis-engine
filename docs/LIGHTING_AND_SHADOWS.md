# Lighting and Shadows

## Ziel

Der Genesis Renderer besitzt eine deterministische Lighting- und Shadow-Pipeline-Foundation. Die Lichtregistrierung unterstützt Directional, Point und Spot Lights und validiert die relevanten Parameter. Lichtquellen werden nach `LightId` deterministisch geordnet.

## Beleuchtung

Unterstützte Lichttypen:

- Directional Light
- Point Light
- Spot Light

Die Lichtregistrierung validiert IDs, Richtungen, Intensität, Reichweite, Finite-Werte und Spot-Winkel. fileciteturn254file0

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

Diese Ebene erzeugt **keine** Vulkan-, D3D12-, Metal- oder OpenGL-Kommandos und reserviert keine GPU-Ressourcen. Sie definiert ausschließlich die Render-Graph-nahe Ausführungsplanung. Die bestehende Backend-Abstraktion bleibt für API-spezifische Ausführung zuständig. fileciteturn257file0

## Datenfluss

`LightRegistry → ShadowConfig → Cascade/Projection Selection → ShadowFramePlan → Backend Shadow Resources → Shadow Depth Pass → Lighting Pass`

Die ersten vier Stufen sind implementiert. Die letzten drei benötigen backend-spezifische GPU-Implementierung.

## Tests

Implementiert:

- Lichtvalidierung
- deterministische Lichtsortierung
- Cascade-Grenzen
- maximale Schattendistanz
- ungültige Shadow-Konfiguration
- deterministische Shadow-Light-Auswahl
- Directional-Light-Cascade-Passplanung
- stabile Pass-Reihenfolge

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
11. Render-Graph-Abhängigkeit `Shadow → Lighting`
12. Vulkan-Ausführung
13. Direct3D 12-Ausführung
14. Metal-Ausführung
15. GPU-Profiling und Speicherbudgetierung

## Status

`LIGHTING_FOUNDATION_IMPLEMENTED`

`SHADOW_PASS_PLANNING_IMPLEMENTED`

`GPU_SHADOW_RENDERING_NOT_ESTABLISHED`

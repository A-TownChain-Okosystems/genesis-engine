# Lighting and Shadows

## Ziel

Genesis Renderer erhält eine deterministische Lighting-/Shadow-Foundation für die drei vorhandenen Lichttypen:

- Directional Light
- Point Light
- Spot Light

Die bestehende Lichtregistrierung validiert IDs, Richtungen, Reichweiten, Intensitäten und Spot-Winkel und sortiert Lichtquellen deterministisch nach `LightId`. fileciteturn254file0

## Schattenmodell

`ShadowConfig` definiert:

- `ShadowMode::None`
- `ShadowMode::Hard`
- `ShadowMode::Pcf`
- Shadow-Map-Auflösung 256 bis 4096
- Depth Bias
- Normal Bias
- maximale Schattendistanz
- 1 bis 8 Cascades

Die Auflösung ist explizit als Backend-unabhängige Policy modelliert. Die eigentlichen GPU-Depth-Targets und Renderpasses gehören weiterhin zum jeweiligen Graphics Backend.

## Directional-Light-Cascades

`directional_cascades()` berechnet deterministische CSM-Splits aus einer Mischung aus logarithmischer und uniformer Aufteilung. Die letzte Cascade wird exakt auf `min(camera_far, max_distance)` begrenzt.

Das reduziert die Auflösungsverschwendung typischer Shadow Maps über große Sichtdistanzen und bildet die Grundlage für Cascaded Shadow Maps.

## Shadow-Caster-Selektion

`shadow_lights()` liefert Shadow-fähige Licht-IDs in deterministischer Reihenfolge. `ShadowCaster` definiert die gemeinsame CPU-seitige Repräsentation für potenzielle Shadow-Caster.

## Architekturgrenze

Die Shadow-Foundation ist bewusst nicht an Vulkan, D3D12 oder Metal gekoppelt. `lib.rs` exportiert Lighting und Shadows als Renderer-API; Backend-spezifische GPU-Ressourcen bleiben in den Backend-Modulen. fileciteturn255file0

## Fehlerklassen

- `InvalidBias`
- `InvalidDistance`
- `InvalidCascadeCount`
- `UnsupportedLight`

## Tests

Implementiert sind Tests für:

- monotone Cascade-Grenzen
- Begrenzung der letzten Cascade
- ungültige Cascade-Konfiguration
- deterministische Shadow-Light-Reihenfolge

## Noch offene Implementierung

Dies ist eine **Foundation**, noch kein vollständiger GPU-Shadow-Renderer. Offen sind insbesondere:

1. echte Depth-Texture-Allokation je Backend
2. Shadow-Renderpasses
3. Directional CSM-Matrices und texel snapping
4. Point-Light-Cubemap-Shadows
5. Spot-Light-Shadow-Maps
6. PCF/PCSS-Auswertung im Shader
7. Kontakt-/Self-Shadowing
8. Cascaded-Shadow-Filtering und Stabilisierung
9. GPU-Culling der Shadow-Caster
10. Shadow-Cache und Update-Frequenzen
11. Render-Graph-Abhängigkeiten für Shadow Pass → Lighting Pass
12. Vulkan/D3D12/Metal-spezifische Ausführung

## Status

`FOUNDATION_IMPLEMENTED / GPU_SHADOW_RENDERING_NOT_ESTABLISHED`

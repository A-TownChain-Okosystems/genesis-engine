<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-GAME-001
  name: genesis-engine
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S1
  criticality: low
-->

# ATC Genesis Engine

[![ATC-COMPLIANCE](https://img.shields.io/badge/ATC-COMPLIANCE-v1.0-green)](./AGENTS.md)

> General-purpose, modulare Game-Engine für ECS, Weltensimulation, Rendering, Physik, Audio, Animation, Networking, AI und Editor-Workflows.

**Project:** `genesis-engine`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0 — A-TownChain-Okosystems`

## Overview

Genesis Engine ist die **general-purpose Game-Development-Plattform** des A-TownChain-Ökosystems. Die Engine stellt wiederverwendbare Runtime-, Simulations-, Tooling- und SDK-Funktionen bereit. `genesis-chronicles` ist ein unabhängiger Consumer/Flagship-Titel und keine technische Voraussetzung für die Engine.

Die kanonische Cargo-Workspace-Struktur besteht aus den folgenden Modulen:

| Modul | Verantwortung |
|---|---|
| `atc-genesis-animation` | Animation und Animationslaufzeit |
| `atc-genesis-assets` | Asset-Verträge, Ressourcen und Content-Pipeline-Basis |
| `atc-genesis-audio` | Audio-Runtime und Audio-Abstraktionen |
| `atc-genesis-ecs` | Entity Component System und World/ECS-Bridge |
| `atc-genesis-physics` | Physik-Abstraktion und Simulation |
| `atc-genesis-platform` | Gemeinsame Primitive und Backend-Interfaces |
| `atc-genesis-renderer` | Rendering-Abstraktion |
| `atc-genesis-ui` | UI-Abstraktionen |
| `atc-genesis-sdk` | Öffentliche Engine-/SDK-Schnittstellen |
| `atc-genesis-ai` | AI-Integration |
| `atc-genesis-network` | Networking und Replikation |
| `atc-genesis-build` | Build- und Packaging-Funktionen |
| `atc-genesis-tools` | Entwickler- und Engine-Tools |
| `atc-genesis-cli` | Kommandozeilenwerkzeuge |
| `atc-genesis-editor` | Editor-Funktionen |
| `atc-genesis-input` | Input-Abstraktionen |
| `atc-genesis-world` | Welt- und Chunk-Simulation |
| `atc-genesis-gameplay` | Generische Gameplay-Systeme |
| `atc-genesis-runtime` | Runtime-Orchestrierung und Subsystem-Lifecycle |

Der **kanonische Runtime-Kern** ist damit `atc-genesis-runtime`. Ein Cargo-Paket `atc-genesis-engine` ist aktuell **kein Workspace-Mitglied**. Historische Dateien unter `modules/atc-genesis-engine/` dürfen nicht mit dem aktuellen Rust-Workspace verwechselt werden und müssen bei der weiteren Migration separat behandelt werden.

Die Engine befindet sich im Rebuild und ist **nicht als Production-Ready oder finaler Releasezustand** zu verstehen.

## Purpose

Genesis Engine ist für die generische Laufzeit- und Simulationsinfrastruktur der Genesis-Spielplattform verantwortlich:

- Engine-Lifecycle und Runtime-Orchestrierung
- ECS-basierte Simulation
- Welt-, Chunk- und Streaming-Infrastruktur
- Gameplay-Systeme
- Rendering, Physik, Audio und Animation
- Input und UI
- AI- und Networking-Integration
- Editor- und Entwicklerwerkzeuge
- SDK- und Build-/Packaging-Infrastruktur

Spielspezifische Logik gehört in das jeweilige Spiel-Repository. Generische Features werden nur über den vorgesehenen Feature-Promotion-Prozess in die Engine übernommen.

## Architecture

### Dependency direction

```text
Platform primitives
       ↓
 ECS / World / Input / Assets
       ↓
Physics / Animation / Audio / Gameplay
       ↓
AI / Network / Renderer / UI
       ↓
     Runtime
       ↓
 Editor / Tools / CLI / Build / SDK
```

Die tatsächlichen Cargo-Abhängigkeiten sind maßgeblich; diese Darstellung ist ein Architekturmodell und ersetzt keine `Cargo.toml`-Definition.

### Runtime data flow

```text
Input
  ↓
Fixed Simulation Tick
  ├── ECS
  ├── Gameplay
  ├── Physics
  ├── Animation
  ├── AI
  └── World / Streaming
  ↓
Authoritative State
  ├── Network replication
  ├── Audio
  └── Render preparation
          ↓
       Renderer
          ↓
       Present
```

### Ecosystem Boundary

```text
ATCLang / ATC-VM / A-TownChain
              │
              │ Chain- und On-Chain-Funktionen
              ▼
       Genesis Integration
              │
              ▼
       Genesis Engine
              │
              ▼
     Genesis Chronicles / Games
```

Die Engine ist keine Blockchain, kein Kernel und kein Ersatz für ATC-VM oder ShivaCore. Chain-seitige Zustandsübergänge und Contracts bleiben an der vorgesehenen Chain-/VM-Grenze.

## Determinism

Deterministische Simulation ist ein explizites Engine-Ziel. Für deterministische Pfade müssen insbesondere folgende Bereiche kontrolliert werden:

- ECS-Iteration und Systemreihenfolge
- RNG und Seeds
- Gameplay-State
- Physik
- Welt-/Chunk-Streaming
- Netzwerk-Ticks
- Serialisierung
- Replay-/State-Verification

Ungeordnete Datenstrukturen dürfen auf einem deterministischen Simulationspfad nicht unkontrolliert die Ausführungsreihenfolge bestimmen.

## AI Boundary

Externe oder asynchrone AI darf den deterministischen Simulationszustand nicht direkt verändern. Der Zielpfad ist:

```text
AI Inference
    ↓
Validated Command
    ↓
Deterministic Simulation
    ↓
State Change
```

## Requirements

- Rust >= 1.75 / Cargo
- Python >= 3.11 für vorhandenes Tooling und historische Sync-Skripte
- Git >= 2.30

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/genesis-engine.git
cd genesis-engine
cargo build --workspace
```

## Testing

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps
```

Security-/Dependency-Prüfungen und Engine-spezifische Gates laufen zusätzlich über GitHub Actions.

Testergebnisse sind Evidence. Ein erfolgreicher Testlauf bedeutet nicht automatisch `AUDITED` oder `PRODUCTION_READY`.

## Development

Entwicklung erfolgt nach den geltenden A-TownChain-Governance- und Repository-Standards. Commits müssen dem Conventional-Commit-Modell entsprechen.

Vor größeren Änderungen sind mindestens `STATUS.md`, `AGENT_MANIFEST.md`, `ARCHITECTURE.md`, `ROADMAP.md` und die relevanten Governance-Dokumente zu prüfen.

## Security

Security Issues dürfen nicht öffentlich über GitHub Issues gemeldet werden. Sicherheitslücken sind über den offiziellen Security-Reporting-Prozess in `SECURITY.md` zu melden.

**Security class:** S1  
**Criticality:** low

## Audit

Der laufende Engineering-Audit wird in folgenden Dateien dokumentiert:

- `docs/ENGINEERING_AUDIT.md` — Audit-Baseline und Evidenzstatus
- `docs/ENGINEERING_AUDIT_FINDINGS.md` — Finding Registry und Remediation-Status

Definition of Done für Findings:

```text
Finding
  → Root Cause
  → Fix
  → Regression Test
  → CI Verification
  → Audit Evidence
  → CLOSED
```

## Documentation

- `ARCHITECTURE.md` — technische Architektur
- `STATUS.md` — aktueller Projektstatus
- `ROADMAP.md` — Entwicklungs-Roadmap
- `docs/ENGINEERING_AUDIT.md` — Audit-Baseline
- `docs/ENGINEERING_AUDIT_FINDINGS.md` — Findings
- `docs/specs/GEN-PROD-001-PRODUCT-STRATEGY.md` — Produktstrategie
- `docs/REPOSITORY_STANDARD.md` — Repository-Standard
- `a-townchain-os-docs` — zentrale Ökosystem-Dokumentation

## Governance

Das Repository folgt dem A-TownChain-Governance-Modell. Architektur- und Governance-Entscheidungen müssen über die vorgesehenen Entscheidungs- und Review-Prozesse erfolgen.

Canonical Standard-IDs werden ausschließlich über die Standards Registry und den dafür definierten Governance-Prozess vergeben. Die aktuelle Taxonomie verwendet Family-scoped IDs der Form `ATC-STD-Fxx-yyy`; bestehende Legacy-IDs bleiben historisch erhalten und werden nicht stillschweigend umnummeriert.

## Standards & Compliance

| Standard | Version | Verwendung |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | Governance Root |
| ATC-STD-README-001 | 1.0.0 | README-Struktur und Metadaten |
| ATC-STD-MD-001 | 1.0.0 | Markdown-Konformität |
| ATC-STD-201 | 1.0.1 | Repository Governance |
| ATC-STD-202 | 1.2.0 | Repository/Entwicklungsanforderungen |
| ATC-STD-203 | 1.0.1 | Security und Release Gates |

Die Tabelle dokumentiert relevante Standards; sie ist keine pauschale Behauptung, dass dieses Entwicklungs-Repository bereits `PRODUCTION_READY` ist.

## Roadmap

Siehe:

- `ROADMAP.md`
- `STATUS.md`
- zentrale Roadmap in `a-townchain-os-docs`
- GitHub Issues & Projects

## Contributing

Beiträge erfolgen über den definierten ATC-Governance-Prozess. Vor einem Merge müssen die für die Änderung relevanten Tests und Validatoren erfolgreich ausgeführt werden.

## License

Apache-2.0 — A-TownChain-Okosystems. Details siehe [`LICENSE`](LICENSE).

## Maintainers

**Organization:** A-TownChain-Okosystems  
**Maintainers:** ShivaCoreDev, aurora-superagent

## Repository Metadata

Maschinenlesbar: siehe HTML-Metadaten-Block im Header gemäß ATC-STD-README-001 §14.  
**Registry-ID:** `ATC-REPO-GAME-001`

## AI Agent Instructions

Für KI-Agenten:

1. Lies `STATUS.md`, `AGENT_MANIFEST.md`, `ARCHITECTURE.md` und `ROADMAP.md` vor größeren Änderungen.
2. Beachte die geltenden ATC-Standards und Repository-Governance.
3. Verwende Conventional Commits.
4. Führe nach Änderungen mindestens `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo test --workspace --all-targets` und `cargo clippy --workspace --all-targets -- -D warnings` aus.
5. Trenne deklarierte Zustände, Testergebnisse und Governance-Evidence strikt voneinander.
6. Verändere keine Chain-/VM-Grenzen, um Engine-Funktionalität zu implementieren.

## Komponenten

- **`components/gff`** — Genesis Franchise Factory (GFF): vollstaendige Ueberfuehrung des
  Quellrepos `genesis-franchise-factory` per `git subtree` (volle Historie, Stand der
  Feature-Branches inklusive AI-Workflow-Katalog). Referenzimplementierung `gff/`
  (Core/DAO/Lifecycle/Workflows/GameFactory), 23 kanonische Factory-Specs `specs/*.atc`
  (AD-20..AD-43), 42 pytest-Tests, eigene Quality Gates (ruff+pytest, Job `gff-pytest`).
  Plattform-Trennungsregel bleibt verbindlich: GFF sitzt UEBER der Engine, keinerlei
  Genesis-Chronicles-Abhaengigkeit. Determinism-Allowlist: `determinism_allowlist.yaml`.

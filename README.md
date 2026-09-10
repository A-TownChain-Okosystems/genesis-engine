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

> Modulare KI-native Game-Engine (ECS, Creatures, World) des A-TownChain-Ökosystems (Layer L6).

**Project:** genesis-engine
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0 — A-TownChain-Okosystems`

## Overview

Genesis Engine ist die zentralisierte Game-Engine des A-TownChain-Ökosystems (Layer L6 in der 23-Repo-Bauhierarchie gemaess AD-026). Sie bietet ein hochmodulares Framework bestehend aus Core Engine, Entity Component System (ECS), Kreaturen-Logik und Weltensimulation (atc-genesis-engine, atc-genesis-ecs, atc-genesis-creatures, atc-genesis-world).

Vault-Restauration (07.09.2026, AD-020/026/027): Inhalt aus dem Wiki-Vault (docs/archive/monorepo-full/) restauriert — vor der Repo-Leerung byte-identisch gesichert.

## Purpose

ATC Genesis Engine provides the canonical game engine implementation for the A-TownChain ecosystem. It is responsible for:

- Ausführung und Bereitstellung der Spiel- und Simulations-Logik auf Layer L6.
- Bereitstellung der vier Kernmodule: `atc-genesis-engine`, `atc-genesis-ecs`, `atc-genesis-creatures` und `atc-genesis-world`.
- Anbindung an das ATCLang / ATVM / ShivaCore Ökosystem und spätere Verknüpfung mit GameFi-Anwendungen (z. B. `genesis-chronicles`).
- Erfüllung des Meilensteins M7 („Spiel läuft: Engine-Loop/ECS stabil") in der Lauffähigkeits-Roadmap (AD-027).

Davon hängen ab: `genesis-chronicles` (L6 GameFi) sowie die Integration im Monorepo `a-townchain-os` (L7).

## Status

**Status:** `development` — Basis für den Rebuild; Gate-Kriterien laut LAUFFAEHIGKEITS_ROADMAP (a-townchain-os-docs/docs/roadmap/).

ATC Compliance: R2 — auditiert am 2026-09-07 (atc-repo-audit; R-Level aus `.atc/repository.yaml`).

## Architecture

### Components

- `atc-genesis-engine` — Core-Engine-Loop, Lifecycle-Management, Pipeline & Subsystem-Orchestrierung.
- `atc-genesis-ecs` — High-Performance Entity Component System.
- `atc-genesis-creatures` — KI- und Kreaturen-Verhaltensmodellierung.
- `atc-genesis-world` — Prozedurale Weltgenerierung, Terrain & Umgebungssimulation.

### Data Flow

Input Events / Engine Tick → `atc-genesis-ecs` (System-Ausführung) → `atc-genesis-creatures` (Verhaltens-Update) → `atc-genesis-world` (Umwelt-Simulation) → Output State / Rendering.

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| atc-genesis-engine | Core Engine Loop | Yes |
| atc-genesis-ecs | Entity Component System | Yes |
| atc-genesis-creatures | Creature Behavior | Yes |
| atc-genesis-world | World & Environment Simulation | Yes |

## Features

- Modulare Rust-basierte Engine-Architektur
- High-Performance ECS Framework
- KI-native Kreatureneigenschaften und Verhaltensbäume
- Prozedurales Terraingenerierungs- und Weltensystem
- Vorbereitung für Multiplayer und ZKP-State-Integrationsprüfungen

## Repository Structure

```text
/
├── docs/       # Dokumentation & Repository-Standards
└── modules/    # Engine-Module (atc-genesis-engine, atc-genesis-ecs, atc-genesis-creatures, atc-genesis-world)
```

## Requirements

- Rust >= 1.75 / Cargo
- Python >= 3.11 (für Tooling & Sync-Skripte)
- Git >= 2.30

## Installation

### Setup

```bash
git clone https://github.com/A-TownChain-Okosystems/genesis-engine.git
cd genesis-engine
cargo build --workspace
```

## Configuration

Die Modulkonfiguration erfolgt über `Cargo.toml` in den jeweiligen Modulverzeichnissen unter `modules/` sowie über Umgebungsvariablen für das Monorepo-Workspace-Syncing.

## Usage

```bash
cargo run --package atc-genesis-engine
```

## Development

Entwicklung erfolgt nach den ATC Governance-Standards (ATC-STD-000 §7 Naming, ATC-STD-201, ATC-STD-202). Commits müssen dem Conventional Commit Standard folgen.

## Testing

Die Test-Suite wird über Cargo ausgeführt:

```bash
cargo test --workspace
```

Expected result: PASS (alle Modultests erfolgreich).

## Security

Security issues must not be disclosed publicly through GitHub Issues. Report security vulnerabilities through the official ATC security reporting process (ATC-STD-203, `SECURITY.md`). S-Klasse S1, Criticality: low.

## Documentation

- `docs/REPOSITORY_STANDARD.md` — Spezifischer Repository-Standard
- `ARCHITECTURE.md` — Technische Architekturübersicht
- `STATUS.md` — Aktueller Projektstatus
- `ROADMAP.md` — Entwicklungs-Roadmap
- Central Docs Hub: `a-townchain-os-docs`

## Governance

This repository is governed according to the A-TownChain Enterprise Governance Framework (ATC-STD-000 v1.2.0, ATC-ENT-001..015). Architekturentscheidungen sind zentral im `DECISIONS_REGISTER` (AD-Nummern) dokumentiert.

## Standards & Compliance

This repository follows applicable A-TownChain standards:

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.0.0 | ✅ |
| ATC-STD-203 | 1.0.0 | ✅ |

## Roadmap

Siehe kanonische Roadmap-Quellen:

- `ROADMAP.md` (Repo-Wurzel, enthält u. a. die 9 VISION-Eskalationsstufen #1-#9)
- `LAUFFAEHIGKEITS_ROADMAP.md` in `a-townchain-os-docs`
- GitHub Issues & Projects

## Contributing

Beiträge erfolgen ausschließlich über den ATC-Governance-Prozess. Details siehe `CONTRIBUTING.md` und `ATC-STD-000`.

## License

Apache-2.0 — A-TownChain-Okosystems (Michael Wroblewski / ShivaCore). Details siehe `LICENSE`.

## Maintainers

**Organization:** A-TownChain-Okosystems
**Maintainers:** ShivaCoreDev, aurora-superagent

## Repository Metadata

Maschinenlesbar: siehe HTML-Metadaten-Block im Header (ATC-STD-README-001 §14). Registry-ID: ATC-REPO-GAME-001.

## AI Agent Instructions

Für KI-Agenten, die an diesem Repository arbeiten:
1. **Standards:** Beachte ATC-STD-000, ATC-STD-README-001, ATC-STD-MD-001, ATC-STD-201, ATC-STD-202, ATC-STD-203.
2. **Workflow:** 
   - Prüfe `STATUS.md` und `AGENT_MANIFEST.md` auf aktuellen Zustand.
   - Lies `ARCHITECTURE.md` und `ROADMAP.md` vor großen Refactorings.
   - Verwende Conventional Commits für alle Änderungen.
   - Stelle sicher, dass nach Änderungen `cargo test --workspace` und die Validators (`check_readme.py`, `check_md.py`) PASS melden.

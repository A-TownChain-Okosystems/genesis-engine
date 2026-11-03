---
document_id: ATC-DOC-ENGINE-CL-001
title: CHANGELOG — genesis-engine
version: 1.0.0
status: active
standard: ATC-STD-MD-001
date: 2026-09-07
---

# Changelog — genesis-engine

## [Unreleased]

### Added
- Deterministic atc-genesis-story-ai narrative runtime.
- Story graph, scenes, chapters, characters, relationships, quests, dialogue, lore, memory, events and timeline contracts.
- Atomic consequence transactions with rollback.
- Deterministic trigger registry and one-shot trigger semantics.
- Provider-neutral AI NarrativeProposal boundary and proposal validation.
- Replay log and non-cryptographic deterministic state fingerprint.
- Genesis Engine integration adapter that prevents unvalidated AI proposals from mutating runtime state.
- Dedicated integration, rollback, determinism, replay and trigger tests.

### Changed
- Workspace now includes modules/atc-genesis-story-ai.


All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-07

### Added
- Standards-Compliance nach ATC-STD-README-001 und ATC-STD-MD-001 hergestellt.
- Kanonische `STATUS.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md` und `AGENTS.md` erstellt.

### Changed
- `README.md` auf ATC-STD-README-001 21-Sektionen-Pflichtstruktur aktualisiert.
- Governance-Metadaten und CI-Alignment.

### Fixed
- Unvollständige Header-Metadaten und fehlende Pflichtdateien behoben.

## [0.0.1] - 2026-09-06

### Added
- Initialer Vault-Import aller vier Engine-Module (`atc-genesis-engine`, `atc-genesis-ecs`, `atc-genesis-creatures`, `atc-genesis-world`).

---
document_id: ATC-DOC-ENGINE-ARCH-001
title: Architecture — genesis-engine
version: 1.0.0
status: active
standard: ATC-STD-MD-001
date: 2026-09-07
---

# Architecture — genesis-engine

> **Technische Architektur der ATC Genesis Engine (Layer L6)**

## Overview

Genesis Engine ist eine modulare, KI-native Game-Engine für das A-TownChain-Ökosystem. Sie ist auf Layer L6 der 23-Repo-Bauhierarchie angesiedelt und bietet performante Abstraktionen für Entity Component Management, Kreaturen-Verhalten und Umwelt-Simulation.

## Modul-Architektur

Das Repository gliedert sich unter `modules/` in vier Kerndomänen:

```text
modules/
├── atc-genesis-engine/       # Core Engine Loop, Event Bus, System Scheduler
├── atc-genesis-ecs/          # High-Performance ECS Architecture
├── atc-genesis-creatures/    # Behavioral AI Trees & Creature Mechanics
└── atc-genesis-world/        # Procedural World Generation & Environment Simulation
```

## Schichtenmodell & Integration

1. **Base Layer (L0-L5):** Nutzung von ATCLang (`atclang`), ShivaCore (`atc-shivacore`), Aurora-AI (`aurora-ai`) und A-TownChain (`a-townchain`).
2. **Engine Layer (L6):** In-Memory Tick Pipeline (`atc-genesis-engine`), Deterministic State Computations via `atc-genesis-ecs`.
3. **Application & GameFi (L6-L7):** Anbindung an GameFi-Titel wie `genesis-chronicles` (Shivamon) und Monorepo-Integration `a-townchain-os`.

## Security & Trust Boundaries

- Die Engine läuft außerhalb des verifizierten Konsens-Kernels (S1 Security Class).
- Konsens- und State-Relevanz werden über Verifizierer an die ATVM (`atc-vm`) übergeben.

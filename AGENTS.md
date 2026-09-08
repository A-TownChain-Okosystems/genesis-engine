---
document_id: ATC-DOC-ENGINE-AGENTS-001
title: AI Agent Instructions — genesis-engine
version: 1.0.0
status: active
standard: ATC-STD-MD-001
date: 2026-09-07
---

# AI Agent Instructions — genesis-engine

> **Anweisungen für KI-Agenten und automatisierte Systeme**

## Identity & Standards

Dieses Repository unterliegt streng der A-TownChain Governance:
- **ATC-STD-000:** Verfassung & Naming Standards (§7)
- **ATC-STD-README-001:** README Quality Gates 01..13
- **ATC-STD-MD-001:** Markdown & Documentation Quality Gates MD-01..10
- **ATC-STD-201 / 202 / 203:** Repository, Naming & Security Standards

## Entry Point Sequence

Bei Aufgaben in diesem Repository halten Sie folgende Reihenfolge ein:
1. `README.md` (Identität, Setup, Standards)
2. `STATUS.md` (Aktueller Systemzustand)
3. `ARCHITECTURE.md` (Systemaufbau & Komponenten)
4. `ROADMAP.md` (Meilensteine & VISION-Issues)
5. `CHANGELOG.md` (Bisherige Versionen)

## Required Workflow

1. **Status prüfen:** Stellen Sie sicher, dass bestehende Tests (`cargo test --workspace`) grün sind.
2. **Änderung durchführen:** Arbeiten Sie nach Modularitätsprinzipien; keine unnötigen Wurzel-Dateien erstellen.
3. **Dokumentieren:** Aktualisieren Sie `CHANGELOG.md` (Keep a Changelog Format) und `STATUS.md` bei Funktionsänderungen.
4. **Validieren:** Führen Sie lokal die Validatoren aus:
   ```bash
   python3 /app/repos/atc-standards/tools/atc-readme-validator/check_readme.py .
   python3 /app/repos/atc-standards/tools/atc-md-validator/check_md.py .
   ```
5. **Commit:** Erstellen Sie Conventional Commits mit dem vorgeschriebenen Agent-Trailer.

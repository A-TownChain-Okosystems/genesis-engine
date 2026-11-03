---
document_id: ATC-DOC-ENGINE-STATUS-001
title: Repository Status — genesis-engine
version: 1.1.0
status: active
standard: ATC-STD-MD-001
date: 2026-11-03
---

# Status — genesis-engine

> **Status:** active (v1.1.0) — snapshot reviewed 2026-11-03

## Repository Status

| Property | Value |
|---|---|
| Repository | genesis-engine |
| Version | 0.1.0 |
| Status | development / monolithic integration |
| Rust Workspace | integrated |
| Security | S1 |
| Documentation | ATC-STD-MD-001 CONFORM |

## Integrated Engine Surface

The repository now contains the monolithic foundations for platform contracts, renderer, physics, audio, animation, assets, UI, input, editor, SDK, gameplay AI, networking, build/package, developer tools and CLI. Existing ATC-based engine/ECS/world/creature modules remain part of the same repository.

## Verification

> **Evidence freshness notice (2026-11-03):** the last successful Test Suite run recorded in the evidence SSOT is `39981967116`, bound to `f95047d63c3ca7304b0e3a003998a15cc09f26ca`. Current main is `3c2be8c11c00bf5b6a1a4de87760004e9c5d35ef`; the historical run does not verify this revision. Current test status remains `not_run` until an exact-commit run and its immutable artifact are reconciled. This status page does not claim `CI_VERIFIED`, `AUDITED` or `PRODUCTION_READY`.

The canonical integration gate is:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The source tree and workspace manifests have been updated, but a GitHub-side workflow result is required before declaring the current revision CI-verified. A successful build/test does not imply `AUDITED` or `PRODUCTION_READY`.

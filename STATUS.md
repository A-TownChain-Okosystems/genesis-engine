---
document_id: ATC-DOC-ENGINE-STATUS-001
title: Repository Status — genesis-engine
version: 1.1.0
status: active
standard: ATC-STD-MD-001
date: 2026-10-09
---

# Status — genesis-engine

> **Status:** active (v1.1.0) — snapshot reviewed 2026-10-09; the underlying status record dates from 2026-09-15

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

> **Evidence freshness notice (2026-10-09):** the integration-surface inventory above is descriptive, not proof that each subsystem is complete. The latest-commit query in this refresh returned no PR-triggered workflow runs for the inspected SHA; this does not establish that no CI exists. The current revision must be checked in Actions before claiming CI verification.


The canonical integration gate is:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The source tree and workspace manifests have been updated, but a GitHub-side workflow result is required before declaring the current revision CI-verified. A successful build/test does not imply `AUDITED` or `PRODUCTION_READY`.

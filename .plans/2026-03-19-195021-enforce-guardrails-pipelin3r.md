# Enforce guardrails on pipelin3r

**Date:** 2026-03-19 19:50
**Task:** Fix all guardrail violations in ~/Projects/websmasher/pipelin3r

## Errors (must fix)
1. 7x R12: Add regex crate bans to deny.toml (root + apps/shedul3r)
2. 1x R26: Add missing_debug_implementations to workspace lints

## Warnings (should fix)
3. 4x R2: Create per-crate clippy.toml for crates missing them
4. R-ARCH-01: shedul3r-bin missing hex arch layers
5. R-ARCH-04: Workspace members not configured in guardrail3.toml
6. R-TEST-*: Mutation testing config
7. R-PUB-*: Package metadata (readme, keywords, categories)
8. R-REL-03: release-plz.toml missing packages

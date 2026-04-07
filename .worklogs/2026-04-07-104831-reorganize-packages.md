# Reorganize packages into grouped folders

**Date:** 2026-04-07 10:48
**Scope:** All packages — directory reorganization

## Summary
Moved 30+ flat packages into grouped folder structure:
- `packages/rs/{family}/` — g3rs checks, ingestion, and workspace-crawl
- `packages/parsers/` — all TOML parser crates
- `packages/shared/` — guardrail3-check-types, reason-policy

Rewrote 104 cross-package path dependencies across packages/ and apps/. All tests pass, app compiles.

## Structure
```
packages/
  rs/
    g3rs-workspace-crawl/
    cargo/ clippy/ deny/ deps/ fmt/ garde/ release/ toolchain/
  parsers/
    cargo-toml-parser/ clippy-toml-parser/ deny-toml-parser/ ...
  shared/
    guardrail3-check-types/ reason-policy/
```

## Fixes during reorg
- deps-config-checks: added assertions anchor in lib.rs
- garde-config-ingestion: removed leftover facade dev-dependencies
- 6 app-domain path references: added 2 extra ../ levels for new depth

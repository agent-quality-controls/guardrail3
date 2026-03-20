# Expand golden fixture to realistic monorepo

**Date:** 2026-03-20 13:09

## Summary
Replaced minimal golden fixture with a realistic 5-app monorepo. Fixed admin app to be pure TS (no hybrid Rust+TS). Updated all test paths.

## Golden fixture structure
- **devctl** — Rust CLI, simple hex arch (5 crates)
- **backend** — Rust server, REST + MCP inbound, DB + queue outbound, hex-in-hex on MCP adapter (13 crates)
- **worker** — Rust async worker, simple hex arch (6 crates)
- **admin** — Next.js, TS hex arch in src/ (no Cargo.toml, skipped by R-ARCH-01)
- **landing** — Next.js content site, TS hex arch in src/ (no Cargo.toml, skipped)
- **packages/shared-types** — Rust library
- **packages/ui-kit** — TypeScript package

## Decision: no hybrid apps
An app is either Rust or TypeScript. If a TS app needs WASM, the WASM crate lives in packages/ as a separate library, not inside the app.

## Key files
- `tests/fixtures/r_arch_01/golden/` — 86 files
- `tests/unit/test_r_arch_01.rs` — 13 tests using devctl (simple) and backend (hex-in-hex)

## Next steps
- Write failing tests for rules 7-11 (workspace enforcement)
- Implement rules 7-11 to make them pass

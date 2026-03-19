# Rustfmt.toml resolution empirically verified

**Date:** 2026-03-19 15:56
**Scope:** coverage/rustfmt.rs

## Verified facts
1. Walk-up from source files, nearest wins, shadows completely
2. Crosses workspace boundaries
3. Intermediate rustfmt.toml shadows for dirs below only
4. Both rustfmt.toml and .rustfmt.toml work
5. .rustfmt.toml (dot) wins over rustfmt.toml (no dot) — opposite of deny
6. cargo fmt -p crate resolves per-crate (per-crate shadowing confirmed)

## Corrected
Previous claim "cargo fmt starts from workspace root only" was WRONG. Verified per-crate resolution with intermediate shadowing.

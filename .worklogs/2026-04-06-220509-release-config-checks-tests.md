# Add tests for g3rs-release-config-checks

**Date:** 2026-04-06 22:05
**Scope:** packages/g3rs-release-config-checks tests + assertions

## Summary
Added 29 tests across all 11 release config checks. Golden fixtures for Cargo.toml, release-plz.toml, and cliff.toml. Each check has golden + failure tests. Assertions crate with per-check assertion helpers.

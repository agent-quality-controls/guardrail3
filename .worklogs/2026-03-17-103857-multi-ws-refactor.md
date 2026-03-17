# Multi-workspace ProjectInfo refactor

**Date:** 2026-03-17 10:38
**Scope:** discover.rs, mod.rs, hex_arch_checks.rs, cargo_lints.rs, release_crate_checks.rs, tests

## Summary
ProjectInfo restructured with Vec<RustWorkspace>. R-ARCH-04 messages include exact TOML example. R26 says which section lint belongs in. R-PUB-08 handles version.workspace = true.

# Fix consistency issues from adversarial attacks

**Date:** 2026-04-06 22:43
**Scope:** Multiple packages — metadata and wiring fixes

## Fixes
1. g3rs-deps-config-checks: wire assertions crate as dev-dependency in runtime
2. g3rs-deps-config-checks, g3rs-garde-ast-checks: add shared = true to facade
3. g3rs-cargo-config-ingestion, g3rs-clippy-config-ingestion, g3rs-fmt-config-ingestion, g3rs-toolchain-config-ingestion: add shared = true to facade

## Known remaining issues (in packages built by delegated agents)
- clippy/fmt checks facades use "runtime"/"types" features instead of "api"
- fmt-config-checks inputs.rs uses .expect() violating expect_used = "deny"
- toolchain ingestion inlines selection in run.rs instead of using select.rs
- 5 ingestion packages missing clippy test lint allows in mod.rs
- Release ingestion missing recovery test, dot-variant preference test, malformed cliff test
- clippy/fmt ingestion use feature-gated deps differently from others

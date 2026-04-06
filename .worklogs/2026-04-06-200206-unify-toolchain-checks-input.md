# Unify toolchain checks input type

**Date:** 2026-04-06 20:02
**Scope:** g3rs-toolchain-config-checks, apps/guardrail3 toolchain family

## Summary
Replaced two split input types (`G3RsToolchainConfigChannelComponentsInput` + `G3RsToolchainConfigMsrvConsistencyInput`) and two check functions (`check_channel_and_components` + `check_msrv_consistency`) with one unified `G3RsToolchainConfigChecksInput` and one `check` function. Cargo.toml fields are `Option` — MSRV check runs when present, skips when absent.

## Why
All other checks packages follow one pattern: one input type (rel paths + parsed types), one check function. The toolchain package broke this pattern by splitting into two types and two functions, which would have forced the ingestion to depend on two checks-input types or create its own result struct. The unified type follows the same pattern as cargo-config, deny-config, clippy-config, etc.

## Key Files
- `packages/g3rs-toolchain-config-checks/crates/types/src/lib.rs` — unified input type
- `packages/g3rs-toolchain-config-checks/crates/runtime/src/run.rs` — single check function
- `packages/g3rs-toolchain-config-checks/src/lib.rs` — updated facade
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/run.rs` — updated app caller

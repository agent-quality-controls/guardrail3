# Garde Config Lane Completion

## Goal

Finish the `garde` package migration by removing the old app fallback behavior from the config lane. The package config lane must own:
- garde dependency presence
- quiet gating of ban rules when `garde` is absent
- warn-level "cannot verify" results when covering clippy config is missing, unreadable, or unparseable

## Approach

1. Add failing package tests for:
   - missing clippy config with `garde` present
   - invalid clippy config with `garde` present
   - `garde` absent with clippy present staying quiet for rules 02-05
   - ingestion returning package input instead of error for present-but-bad clippy
2. Replace the optional `clippy_rel_path`/`clippy` pair with an explicit clippy input state in `g3rs-garde-types`.
3. Rewire `g3rs-garde-config-checks` to:
   - always run config 01
   - short-circuit 02-05 when `garde` is absent
   - emit warn-level unverifiable results for missing/invalid clippy
   - run normal 02-05 checks only when clippy parsed successfully
4. Rewire `g3rs-garde-ingestion` to preserve clippy state instead of erroring on clippy read/parse failures.
5. Update stale docs that still describe the old app bridge or old malformed-input ownership.

## Key decisions

- Keep Cargo.toml missing/unreadable/unparseable as ingestion errors.
  - Why: without parsed cargo the config lane cannot determine applicability or dependency presence.
- Model clippy state explicitly instead of using ad hoc fallback in the runner.
  - Why: the package boundary should own its own missing/invalid config behavior.
- Keep this in config, not filetree.
  - Why: the behavior is about verifying config contents or reporting that config verification is impossible.

## Files to modify

- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/rs_garde_config_0{2,3,4,5}_*/rule.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/**/rule_tests/*`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/*`
- `packages/rs/garde/g3rs-garde-*/README.md`
- `packages/rs/garde/g3rs-garde-*/TODO.md`

# Garde Config Lane Completion

## Summary

Completed the remaining `garde` package migration by moving clippy missing/invalid handling and no-garde gating into the package config lane. The config package now owns `g3rs-garde/dependency-present..05` end to end, while the source package remains the owner of `g3rs-garde/ast-01-struct-derive-validate..08` and `g3rs-garde/input-failures`.

## Decisions made

- Replaced the loose optional clippy pair with an explicit clippy input state enum.
  - Why: the package boundary needed to distinguish parsed, missing, and invalid clippy input without falling back to old app code.
- Kept Cargo.toml missing/unreadable/unparseable as ingestion errors.
  - Why: the config lane cannot safely determine dependency presence or applicability without parsed cargo.
- Kept missing or invalid clippy as config-lane warnings, not ingestion errors.
  - Why: that matches the old rule intent for 02-05 and removes the old app bridge.
- Short-circuited config ban rules when `garde` is absent.
  - Why: the ban rules are only meaningful when the workspace actually uses garde.
- Updated package docs to describe the package model instead of the old app bridge.
  - Why: the old README/TODO text was now lying about package ownership.

## Key files for context

- `.plans/2026-04-12-130033-garde-audit.md`
- `.plans/2026-04-12-130351-garde-config-lane-completion.md`
- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps

- Re-audit `garde` against the old app inventory once more if we want an explicit final parity verdict in the session transcript.
- If the top-level package runner eventually needs guardrail config enable/disable routing, handle that above the family packages rather than pushing app-style routing back into the package lanes.

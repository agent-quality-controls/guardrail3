# Finish RS-TEST Clippy Family

**Date:** 2026-03-29 15:59
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib_tests/mod.rs`

## Summary
Removed the last forbidden `test_support`-local harness from the `clippy` family. The family now validates cleanly under `RS-TEST`, its library suite still passes, and a temp-copy regression proves that reintroducing `test_support/src/lib_tests/mod.rs` still triggers `RS-TEST-03`.

## Context & Problem
After the larger `code` family sweep, `clippy` still had one lingering `RS-TEST-03` error:
- `test_support/src/lib_tests/mod.rs` was a root-local test harness under the `test_support` crate

That shape is no longer allowed. `RS-TEST-03` requires test harnesses to live under the runtime/assertions split, not as standalone `test_support` crate unit tests.

## Decisions Made

### Delete the stray `test_support` harness instead of relocating it
- **Chose:** remove `#[cfg(test)] mod lib_tests;` from `test_support/src/lib.rs` and delete `test_support/src/lib_tests/mod.rs`.
- **Why:** the deleted test was redundant. Runtime-side parity and golden tests already exercise `build_fixture_clippy_toml` across method bans, type bans, macro bans, thresholds, and test relaxations. Keeping an extra `test_support`-local harness added no unique coverage but kept the family red.
- **Alternatives considered:**
  - move the test into a new runtime-side owner — rejected because it would create duplicate parity coverage for no real gain
  - relax `RS-TEST-03` for `test_support` unit tests — rejected because it reopens the exact structural loophole the repo is tightening

### Prove the old shape is still rejected
- **Chose:** run a temp-copy attack that reintroduced `test_support/src/lib_tests/mod.rs`.
- **Why:** the family now passes because the forbidden harness is gone, not because the rule stopped caring.
- **Alternatives considered:**
  - rely only on the clean validator result — rejected because it does not prove the regression shape still fails

## Architectural Notes
This commit sharpens the intended role of `test_support` in migrated families:
- `test_support` provides reusable fixture/setup code
- it does not own test harnesses
- runtime sidecars remain the legal proof sites under `RS-TEST`

## Information Sources
- Live family validation:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
- Family tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
- Existing runtime parity coverage:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_04_missing_method_ban_tests/parity.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/parity.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations_tests/parity.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_19_unknown_keys_tests/parity.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_20_macro_bans_tests/parity.rs`

## Open Questions / Future Considerations
- The repo-root `RS-TEST` backlog is now dominated by `hooks-shared`, `deny`, `hooks-rs`, `hexarch`, and `arch`.
- There is unrelated dirty work outside `clippy` in `release`, `project-tree`, and the `test` family runtime that was intentionally not included here.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — `test_support` crate surface after removal of the forbidden local harness
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_19_unknown_keys_tests/parity.rs` — representative runtime parity coverage proving generated fixture expectations already exist
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations_tests/parity.rs` — representative runtime-side generator parity test
- `.worklogs/2026-03-29-155441-finish-rs-test-code-family.md` — immediately preceding `RS-TEST` family checkpoint

## Next Steps / Continuation Plan
1. Commit only the `clippy` family cleanup and this worklog.
2. Rerun repo-root `RS-TEST` to refresh the next-largest family bucket after `clippy` drops out.
3. Choose the next family based on effort-to-yield:
   - `deny` and `arch` are already in the split family shape and likely cheaper than `hooks-shared`
   - `hooks-shared` remains the larger but structurally heavier migration
4. Keep unrelated dirty files out of the next family checkpoint as well.

# Start RS-RELEASE Test Split

**Date:** 2026-03-29 12:59
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/release/**`

## Summary
Started the `RS-TEST` structural migration for the `release` family by adding sibling `assertions` and `test_support` crates, moving shared test helpers out of the runtime root, and rewiring the sidecar tests to use owned assertions modules. This commit is intentionally a structural checkpoint: `cargo test` for the family is green, but the stricter `RS-TEST-16` checker now surfaces 221 remaining semantic assertions that still need to move out of sidecars.

## Context & Problem
`release` was one of the largest remaining `RS-TEST` violators. The family still relied on a root-local `test_support.rs` blob and many sidecars directly asserted guardrail result semantics inline. Earlier work on `RS-TEST-16` hardened proof detection by traversing assertion macro arguments, which made the existing debt in `release` visible instead of silently accepted.

The immediate goal here was to land the mechanical package split first so the remaining semantic extraction work can proceed on top of an honest family shape. Keeping this separate from the later `RS-TEST-16` cleanup makes the migration easier to reason about and easier to revert if needed.

## Decisions Made

### Split `release` into runtime-owned companion crates
- **Chose:** add sibling `assertions` and `test_support` crates under `apps/guardrail3/crates/app/rs/families/release/`, and register them in the app workspace.
- **Why:** this is the required `RS-TEST` companion structure and gives the sidecars a family-owned place to move reusable semantic assertions without pushing test code into production crates.
- **Alternatives considered:**
  - Keep using `src/test_support.rs` in the runtime crate — rejected because it preserves the old blob pattern that `RS-TEST-03` is trying to eliminate.
  - Hoist assertions into some higher-level shared support crate — rejected because the helpers need to stay family-owned and coupled to release semantics.

### Keep this commit as a structural checkpoint, not a fake “fully clean” migration
- **Chose:** commit the crate split and sidecar rewiring now, while documenting the remaining `RS-TEST-16` violations explicitly.
- **Why:** local family tests already pass, and the remaining work is a separate semantic extraction sweep. Bundling both into one opaque mega-commit would make the transition harder to audit.
- **Alternatives considered:**
  - Wait to commit until every `RS-TEST-16` hit is fixed — rejected because the worktree already contains a coherent structural migration worth preserving.
  - Revert the stricter checker behavior temporarily — rejected because the new findings are real and should stay visible.

## Architectural Notes
The `release` family now follows the same companion-crate model used elsewhere:
- `src/` stays the runtime crate with rule implementations and sidecar modules.
- `assertions/` owns reusable proof-bearing checks for rule results.
- `test_support/` owns generic fixture and helper setup.

This does not make the family `RS-TEST` clean yet. The remaining blocker is semantic ownership: many sidecars still directly inspect result IDs/messages/severities instead of delegating those checks into the sibling assertions crate.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/release/**` — live family code and tests being migrated
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — current `RS-TEST-16` rule contract
- `.worklogs/2026-03-29-125544-harden-rs-test-proof-detection.md` — parser hardening that made the remaining release debt visible
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-release --lib`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/release --family test --inventory --format json`

## Open Questions / Future Considerations
- The family still has 221 live `RS-TEST-16` errors after this checkpoint.
- `Cargo.lock` has separate uncommitted churn outside this structural bucket and was intentionally left out of this commit.
- Some assertion modules are still thin wrappers; they will likely need to grow richer family-specific helpers as the sidecar extraction continues.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/release/Cargo.toml` — runtime crate manifest and family-level dependency wiring
- `apps/guardrail3/crates/app/rs/families/release/assertions/src/lib.rs` — assertion module surface for the family
- `apps/guardrail3/crates/app/rs/families/release/assertions/src/common.rs` — shared assertion helpers used across rule modules
- `apps/guardrail3/crates/app/rs/families/release/test_support/src/lib.rs` — generic release family test fixtures/helpers
- `apps/guardrail3/crates/app/rs/families/release/src/test_fixtures.rs` — runtime-local fixture bridge used by sidecars
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — enforcement rule that now catches the remaining debt
- `.worklogs/2026-03-29-125544-harden-rs-test-proof-detection.md` — why `release` suddenly shows much more `RS-TEST-16` fallout

## Next Steps / Continuation Plan
1. Use the live `RS-TEST-16` output for `families/release` as the migration checklist and move result-shape assertions out of each sidecar into the matching module under `assertions/src/`.
2. Re-run `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-release --lib` after each batch to keep the family green while the sidecars shrink.
3. Re-run `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/release --family test --inventory --format json` until the family is clean.
4. Only after `release` is `RS-TEST` clean, run the adversarial temp-family attack again to confirm the stricter parser still catches hidden semantic assertions and no new bypasses appeared.

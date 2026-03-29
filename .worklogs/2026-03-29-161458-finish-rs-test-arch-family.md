# Finish RS-TEST Arch Family

**Date:** 2026-03-29 16:14
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/*`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_*_tests/*.rs`

## Summary
Completed the `RS-TEST` cleanup for the `arch` family by moving the remaining “quiet” proof checks fully into the sibling assertions crate and fixing the assertions crate root to satisfy `unused-crate-dependencies`. The family now builds cleanly, validates cleanly under the `test` family, and the attack pass still catches the old sidecar-owned proof shape.

## Context & Problem
After the `deny` commit, repo-root `RS-TEST` still had a compact `arch` bucket. Family-local validation for `apps/guardrail3/crates/app/rs/families/arch` showed 21 errors, all `RS-TEST-16`, and all of them came from a single pattern: runtime sidecars imported the sibling assertions crate but still called `assertions::error_results(...).is_empty()` or `assertions::info_results(...).is_empty()` and made the semantic “quiet” decision locally.

At the same time, `cargo test -p guardrail3-app-rs-family-arch --lib` failed before even reaching the runtime tests because the assertions crate carried three dependencies that existed for ownership parity but were not referenced, and the workspace enforces `-D unused-crate-dependencies`.

## Decisions Made

### Move quiet/no-hit proof into the assertions crate
- **Chose:** Add `assert_no_error_files(...)` to the `RS-ARCH-01` through `RS-ARCH-07` assertions modules and `assert_no_info_files(...)` to `RS-ARCH-08`, then rewrite the flagged sidecars to call those owned helpers.
- **Why:** The sidecars were already using the assertions crate; the remaining violation was that they still interpreted filtered result vectors locally. Adding explicit no-hit helpers keeps the proof logic in the owned assertions crate and satisfies `RS-TEST-16` without changing the runtime rule behavior.
- **Alternatives considered:**
  - Leave `error_results(...).is_empty()` in sidecars and weaken `RS-TEST-16` — rejected because it would codify the exact pattern the rule is meant to prevent.
  - Replace all `arch` assertions modules with a new shared macro/common helper layer — rejected for this pass because the family only has eight modules and the direct helper addition was smaller and easier to audit.

### Satisfy lint-only dependency ownership in the assertions crate root
- **Chose:** Add `use guardrail3_app_rs_family_arch as _;`, `use guardrail3_domain_project_tree as _;`, and `use test_support as _;` at the assertions crate root.
- **Why:** The dependencies are intentionally part of the crate boundary, but the crate root did not reference them and the workspace treats unused dependencies as hard errors. Using `as _` preserves the declared dependencies and unblocks the library target without changing public API or tests.
- **Alternatives considered:**
  - Delete the dependencies from `Cargo.toml` — rejected because this pass was about `RS-TEST`, not reevaluating the family’s crate-edge declarations.
  - Scatter dummy uses through individual modules — rejected because the crate root is the cleanest single place to satisfy the lint.

## Architectural Notes
This family uses a simpler assertions layout than `deny` or `code`: each rule module exposes file-set helpers like `assert_error_files(...)` rather than richer finding constructors. The missing piece was an owned “quiet” assertion helper. Once that existed, the sidecars no longer had to inspect result sets locally and the family fit the same proof-ownership model as the other migrated families.

The compile fix is architectural as well: if a sibling assertions crate declares dependency ownership under strict linting, the crate root has to make that ownership explicit even when the modules themselves don’t happen to reference every dependency.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/*.rs` — existing assertion module surface and repetition pattern
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml` — dependency declarations causing the compile failure
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_*_tests/*.rs` — flagged sidecars using `error_results(...).is_empty()` / `info_results(...).is_empty()`
- `.worklogs/2026-03-29-160952-finish-rs-test-deny-family.md` — prior family in the same sweep and the current proof-ownership expectations

## Open Questions / Future Considerations
- Repo-root `RS-TEST` still has broader `RS-TEST-03` fallout in other families and some non-family app-local paths; this commit only clears the contained `arch` bucket.
- The `arch` assertions crate still has duplicated helper structure across modules. If the family evolves further, a shared internal helper or macro may be worth introducing, but it was not necessary for this cleanup.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/lib.rs` — crate-root dependency ownership and module exports
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_02_no_misplaced_roots.rs` — representative new `assert_no_error_files(...)` helper
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots_tests/false_positives.rs` — representative migrated no-hit sidecar
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_08_auxiliary_roots_declared_tests/golden.rs` — representative `info`-quiet migration
- `.worklogs/2026-03-29-160952-finish-rs-test-deny-family.md` — previous family-local proof ownership cleanup

## Next Steps / Continuation Plan
1. Re-run repo-root `RS-TEST` after this `arch` commit and sort the remaining errors by family/path class.
2. Separate the remaining work into:
   - family-local `RS-TEST-03` fallout inside already-migrated families
   - non-family app-local `RS-TEST-02` / `RS-TEST-03` debt under `crates/app/rs/*`
3. Keep staging narrowly, because the long-lived dirty `release` and `project-tree` edits are still present and should not be bundled into these `RS-TEST` cleanup commits.

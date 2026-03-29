# Finish RS-TEST Garde Family

**Date:** 2026-03-29 15:22
**Scope:** `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/**`, `apps/guardrail3/crates/app/rs/families/garde/crates/assertions/src/**`

## Summary
Completed the `RS-TEST` migration for the `garde` family by removing the old runtime-local test fixture hub, moving reusable semantic proof into sibling assertions modules, and wiring each rule sidecar to owned wrappers instead of direct result-shape assertions. The family now validates clean under `--family test`, its unit suite is green, and an adversarial temp-copy pass proves the checker still catches the two shapes this sweep removed.

## Context & Problem
After `release` was finished, `garde` was the next largest `RS-TEST` bucket. The remaining failures were a mix of:
- `RS-TEST-16` sidecars still asserting rule IDs, severities, titles, and messages directly
- `RS-TEST-03` violations from assertions modules reaching sibling local modules
- leftover runtime-root helper structure from the deleted `test_fixtures.rs` pattern

The goal was not just to silence the family locally, but to make the family self-host honestly under the stricter `RS-TEST` rules already landing at repo root.

## Decisions Made

### Replace the runtime-local fixture hub with rule-local wrappers
- **Chose:** delete `crates/runtime/src/test_fixtures.rs` and inline small test-only wrappers into each rule file and `facts.rs`
- **Why:** the shared runtime fixture hub let sidecars and assertions reach too much local orchestration. Per-rule wrappers keep the family runnable in tests without giving sidecars a generic escape hatch.
- **Alternatives considered:**
  - keep `test_fixtures.rs` and whitelist it in `RS-TEST` — rejected because it weakens the rule instead of fixing ownership
  - move route/config helpers into `test_support` — rejected because `RS-TEST-18` correctly treats that as semantic/runtime leakage into generic fixture support

### Make assertions modules the only owners of semantic proof
- **Chose:** add shared assertions helpers in `crates/assertions/src/common.rs` and convert each rule assertions module to expose semantic proof helpers used by sidecars
- **Why:** `RS-TEST-16` is specifically about stopping sidecars from owning result-shape semantics. The common helper centralizes matching logic while keeping rule-specific proof in owned assertions modules.
- **Alternatives considered:**
  - leave result filtering in sidecars and only hide repeated literals — rejected because the semantic ownership violation remains
  - create one family-wide assertions mega-module — rejected because it loses one-rule/one-owned-proof structure

### Keep assertions modules off local private imports
- **Chose:** remove `use crate::...` imports between assertions modules and use shared `common` helper calls instead
- **Why:** `RS-TEST-03` is supposed to keep assertions modules from becoming a second private orchestration layer. Cross-rule quiet checks needed to be expressed without local-private imports.
- **Alternatives considered:**
  - keep sibling assertions imports and relax `RS-TEST-03` — rejected because the repo is explicitly tightening, not loosening, assertions ownership
  - duplicate whole proof logic per module — rejected because the shared quiet-check helper is simpler and keeps semantics explicit

## Architectural Notes
`garde` now follows the intended component test shape:
- runtime sidecars own setup and family execution only
- sibling assertions modules own semantic proof
- `test_support` stays generic and root-agnostic
- runtime no longer exposes a family-wide helper module that all tests can tunnel through

This also makes the family a better specimen for future `RS-TEST` migrations in other rule-heavy families with multi-root behavior.

## Information Sources
- `.plans/todo/checks/rs/garde.md` — current garde rule contract and multi-root expectations
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — ownership restrictions for assertions/runtime/test-support
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — semantic-proof ownership contract
- `.worklogs/2026-03-29-142354-finish-rs-test-release-family.md` — prior specimen for the release-family migration pattern

## Open Questions / Future Considerations
- Repo-root `RS-TEST` still has larger remaining buckets outside `garde`, especially `deps`, `code`, `hooks-shared`, and `deny`.
- The runtime wrappers inserted into each garde rule file are intentionally narrow, but the broader question of a universal helper pattern for non-family components is still open.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/lib.rs` — family orchestrator and remaining family-wide module surface
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/facts.rs` — root discovery facts plus test-only route wrapper used by `facts_tests`
- `apps/guardrail3/crates/app/rs/families/garde/crates/assertions/src/common.rs` — shared result matching helpers/macros for garde assertions
- `apps/guardrail3/crates/app/rs/families/garde/crates/assertions/src/rs_garde_11_field_level_constraints.rs` — representative cross-rule quiet proof without local-private imports
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/rs_garde_12_nested_validation_dive_tests/golden.rs` — representative sidecar after `RS-TEST-16` migration
- `.worklogs/2026-03-29-142354-finish-rs-test-release-family.md` — previous family-level migration pattern that informed this sweep

## Next Steps / Continuation Plan
1. Commit the `garde` family changes only, keeping unrelated repo-root work out of the commit.
2. Rerun repo-root `RS-TEST` to get fresh family counts after the `garde` drop.
3. Move to the next largest `RS-TEST` family bucket and repeat the same sequence: family validator clean, unit tests green, adversarial temp-copy pass, then commit.
4. After the next family lands, rerun repo-root `RS-TEST` again before starting any `RS-CODE` work so overlapping files are not edited out of order.

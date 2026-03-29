# Finish RS-TEST Deps Family

**Date:** 2026-03-29 15:37
**Scope:** `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/**`, `apps/guardrail3/crates/app/rs/families/deps/crates/assertions/src/**`

## Summary
Completed the `RS-TEST` migration for the `deps` family by removing the runtime-root `test_harness` module, adding rule-local test wrappers, converting assertions modules to owned facades, and moving all remaining semantic result checks out of sidecars. The family now passes `cargo test` and validates clean under `--family test`, and a temp-copy attack still catches the old sidecar-boundary and direct-summary regression shapes.

## Context & Problem
After `release` and `garde`, `deps` was the next largest `RS-TEST` bucket. Its failures were concentrated in four patterns:
- `RS-TEST-02`: rule files still declared `mod tests;` ad hoc instead of named sidecar modules
- `RS-TEST-03`: sidecar `mod.rs` files tunneled through `crate::test_harness`, and assertions modules all imported `super::common`
- `RS-TEST-07`: goldens re-exported assertions helpers through sidecar `mod.rs`, which stopped the checker from recognizing a real owned proof site
- `RS-TEST-16`: several sidecars still inspected `result.id`, `result.severity`, `result.message`, and exact summary tuples directly

Unlike `garde`, the `deps` family already had a sibling assertions crate, so the work was mostly about removing the remaining crate-root test hub and making sidecars talk only to rule-local wrappers plus owned assertions modules.

## Decisions Made

### Delete the shared runtime `test_harness` module
- **Chose:** remove `crates/runtime/src/test_harness.rs` and add narrow test-only wrappers inside each rule file
- **Why:** `test_harness` forced every sidecar through a crate-root helper module, which is exactly the kind of boundary leak `RS-TEST-03` is meant to catch.
- **Alternatives considered:**
  - keep `test_harness` and relax `RS-TEST-03` — rejected because it preserves the loophole instead of fixing ownership
  - move the harness into `test_support` — rejected because the helpers depend on runtime facts and route construction, which would leak production semantics into generic support

### Make assertions modules direct owned facades
- **Chose:** replace the hand-written `super::common` wrappers with a shared `define_rule_assertions!` macro plus explicit rule-local helpers where needed
- **Why:** the previous modules were still local-private imports from the checker’s point of view. Converting them to direct owned assertion facades removed the `RS-TEST-03` violations and gave sidecars a stable proof surface.
- **Alternatives considered:**
  - leave the repeated wrappers per file — rejected because they still imported `super::common` and kept failing the rule
  - collapse everything into one family-level assertions module — rejected because it weakens one-rule/one-owned-proof traceability

### Keep cross-rule exactness assertions inside owned rule assertions
- **Chose:** add exactness helpers to `RS-DEPS-01..04` assertions and a broad dependency-routing helper to `RS-DEPS-05`
- **Why:** those tests intentionally reason about multiple rule IDs at once, but sidecars are not allowed to import sibling assertions modules or inspect result IDs directly.
- **Alternatives considered:**
  - let sidecars import multiple sibling assertions modules — rejected because `RS-TEST-03` correctly forbids sibling assertions reach-through
  - drop the cross-rule exactness tests — rejected because those attacks are real and useful

### For same-rule summary tests, remove local assert macros instead of over-abstracting the setup
- **Chose:** leave scenario-local summary extraction in sidecars where useful, but replace local `assert_eq!` with calls into owned assertions helpers such as `assert_rule_results`, `assert_rule_quiet`, or `assert_summary`
- **Why:** `RS-TEST-16` is about semantic ownership, not about banning all local collection logic. Once the proof site moved into the assertions crate, the remaining setup stayed readable and no longer violated the rule.
- **Alternatives considered:**
  - fully inline every expectation into custom assertion helpers — rejected because the resulting helpers became harder to read than the tests they replaced
  - keep local `assert_eq!` and accept the violations — rejected for obvious reasons

## Architectural Notes
`deps` is now aligned with the intended runtime/assertions/test_support split:
- each rule file owns only the tiny test-only wrappers that sidecars need
- sidecar `mod.rs` files re-export only owned rule-local wrappers and generic `test_support` helpers
- sidecars import the owned assertions crate directly when they need semantic proof
- cross-rule semantic assertions live in the owning rule’s assertions module, not in sidecars and not in a crate-root harness

The family is also a useful specimen for config-heavy families where many tests build facts directly rather than always running through full family orchestration.

## Information Sources
- `.plans/todo/checks/rs/deps.md` — deps family contract and root/fail-closed expectations
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — runtime/assertions ownership constraints
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs` — proof-site detection behavior
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — semantic-proof ownership rule
- `.worklogs/2026-03-29-152209-finish-rs-test-garde-family.md` — prior family migration pattern that informed this sweep

## Open Questions / Future Considerations
- Repo-root `RS-TEST` still has larger remaining buckets after `deps`, especially `code`, `hooks-shared`, and `deny`.
- The current per-rule wrapper pattern is repetitive; if more families need the same fact-building helpers, a stricter reusable pattern may be worth designing later, but not by weakening `RS-TEST`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/lib.rs` — family orchestrator after removal of the crate-root test harness
- `apps/guardrail3/crates/app/rs/families/deps/crates/assertions/src/common.rs` — shared deps assertions primitives and macro
- `apps/guardrail3/crates/app/rs/families/deps/crates/assertions/src/rs_deps_05_dependencies_allowlisted.rs` — representative cross-rule assertion ownership helper
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/rs_deps_01_cargo_deny_installed.rs` — representative rule-local test wrappers replacing the shared harness
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/rs_deps_10_gitignore_not_ignoring_cargo_lock_tests/precedence.rs` — representative same-rule summary test after sidecar semantic proof migration
- `.worklogs/2026-03-29-152209-finish-rs-test-garde-family.md` — previous family-level migration specimen

## Next Steps / Continuation Plan
1. Commit the `deps` family changes only, including this worklog.
2. Rerun repo-root `RS-TEST` to refresh the next largest family bucket after `deps` drops out.
3. Move directly to the next family with the same sequence: family validator clean, unit tests green, temp-copy adversarial pass, then commit.
4. Avoid mixing `RS-CODE` work into files that still belong to the remaining `RS-TEST` sweeps.

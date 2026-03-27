# Tighten Clippy Test Boundaries

**Date:** 2026-03-27 20:43
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/{crates/assertions,crates/runtime,test_support}`

## Summary
Tightened the `rs/clippy` self-hosted test boundary without changing family rule behavior. This slice removed the assertions-local helper shortcut, replaced the canned zero-argument clippy fixture helper with a parameterized builder, and cleaned up the immediate test-structure outliers so the remaining `RS-TEST` failures are concentrated in the unresolved rule clusters rather than mixed boundary noise.

## Context & Problem
`rs/clippy` had already been moved into the nested workspace shape, but its first assertions extraction still had two architectural leaks:
- threshold assertion modules depended on a local `crate::common` helper, which `RS-TEST-03` correctly treats as assertions reaching into local private code
- family-local `test_support` exported `canonical_clippy_toml()`, a zero-argument canned semantic helper that `RS-TEST-18` correctly rejects

There were also small outliers obscuring the main migration work:
- `facts_tests/mod.rs` still depended on runtime-local `test_support` rather than the owner module
- `test_support` self-tests still used the stale `test_support_tests` sidecar name
- runtime `lib.rs` still carried a test-only helper module shape that needs to be eliminated, but the clean fix requires broader sidecar migration than this slice

The goal here was not to fix all remaining `RS-TEST` debt in `rs/clippy`, but to remove the easy boundary bugs so the next migration chunk can focus on the coherent rule groups (`04..08`, then `12..20`, then coverage).

## Decisions Made

### Remove the assertions-local `common.rs`
- **Chose:** Inline the threshold assertion proofs directly into each per-rule assertions module and delete `crates/assertions/src/common.rs`.
- **Why:** `RS-TEST-03` is right to reject `crate::common` inside assertions modules. A helper file inside the same assertions crate is still local private code, and proof recognition is cleaner when the assertion macros live in the exported rule module itself.
- **Alternatives considered:**
  - Add a new `assertions_common` crate — rejected for now because the shared helper surface was tiny and would have added another architectural primitive before the family is stable.
  - Relax `RS-TEST-03` for `crate::common` — rejected because that would just reopen the same loophole in another spelling.

### Replace the canned canonical TOML helper with a parameterized builder
- **Chose:** Replace `canonical_clippy_toml()` with `build_fixture_clippy_toml(profile_name, is_pure_layer, garde_enabled, extra_methods, extra_types)`.
- **Why:** The rule is specifically about canned semantic helpers in shared `test_support`. A parameterized builder still supports tests, but it no longer exports one blessed canned baseline as a zero-argument public helper.
- **Alternatives considered:**
  - Keep the zero-arg helper and weaken `RS-TEST-18` — rejected because the rule is correct.
  - Inline TOML in every test immediately — rejected for this slice because it would create a lot of noisy churn before the bigger rule-cluster migrations.

### Isolate the non-rule outliers instead of mixing them into the rule migrations
- **Chose:** Fix the `facts_tests` ownership path and rename `test_support` self-tests to `lib_tests`, while leaving the larger rule clusters for later passes.
- **Why:** These were small structural errors that were obscuring the remaining real work. Clearing them now makes the next migration order much clearer.
- **Alternatives considered:**
  - Ignore the outliers until after the big rule clusters — rejected because they would keep polluting the `RS-TEST` inventory and make progress harder to read.

### Move runtime test helpers out of `lib.rs`, even though the full local-test-support cleanup is still pending
- **Chose:** Reactivate `crates/runtime/src/test_support.rs` and remove the inline helper body from `lib.rs`.
- **Why:** Inline `#[cfg(test)] mod ... { ... }` in `lib.rs` was a direct `RS-TEST-01` violation. Even though the family still needs to migrate callers off the runtime-local helper module entirely, moving the body out of `lib.rs` is the correct intermediate shape.
- **Alternatives considered:**
  - Keep the inline helper and accept the `RS-TEST-01` violation until the full migration — rejected because it preserved the exact pattern the family is supposed to eliminate.

## Architectural Notes
- `rs/clippy` still has a runtime-local `test_support` module. That is no longer inline in `lib.rs`, but it is still the main remaining boundary smell in the family.
- The remaining `RS-TEST` inventory is now much cleaner:
  - the threshold assertion cluster no longer has local-assertions import debt
  - the generic `test_support` canned-helper violation is gone
  - the grouped `facts_tests` path is cleaner, though the family still needs a proper reusable assertions surface for real rule clusters
- The next real work should be on coherent rule groups, not more one-off cleanup:
  1. `RS-CLIPPY-04..08`
  2. `RS-CLIPPY-12..20`
  3. `RS-CLIPPY-01`

## Information Sources
- Existing `rs/clippy` workspace baseline from:
  - `.worklogs/2026-03-27-202045-stabilize-rs-clippy-workspace.md`
  - `.worklogs/2026-03-27-202948-extract-clippy-threshold-assertions.md`
- Current `RS-TEST` contract:
  - `apps/guardrail3/crates/app/rs/families/test/README.md`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs`
- Current clippy family shape:
  - `apps/guardrail3/crates/app/rs/families/clippy/README.md`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/test_support.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs`

## Open Questions / Future Considerations
- The runtime-local `test_support` module still needs to disappear as the remaining sidecars migrate to owner-module helpers plus external `test_support`.
- `facts.rs` now has a placeholder assertions module only. If `facts` keeps tests long-term, it may need a real assertions API rather than just a structural placeholder.
- The next migration chunk should decide whether the `04..08` parity checks keep local pure-data assertions or move fully into per-rule assertions modules.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs` — family entrypoint and current runtime test-support inclusion point
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/test_support.rs` — still-active runtime-local helper surface that remaining sidecars depend on
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — generic external test-support crate after canned helper removal
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/lib.rs` — current assertions surface, including threshold modules and `facts`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — owner helper added for `facts_tests`
- `.worklogs/2026-03-27-202045-stabilize-rs-clippy-workspace.md` — workspace split baseline
- `.worklogs/2026-03-27-202948-extract-clippy-threshold-assertions.md` — first successful rule-cluster extraction pattern

## Next Steps / Continuation Plan
1. Migrate `RS-CLIPPY-04..08` as the next coherent rule cluster:
   - add sibling assertions modules under `crates/assertions/src`
   - add owner-module `run_for_tests(...)` helpers in the runtime rule files
   - move sidecars off `super::super::super::test_support` and off direct `CheckResult` inspection
2. After `04..08`, rerun `rs validate ... --family test` on the clippy family and confirm the remaining failures collapse mostly into `12..20`, `01`, and the last runtime-local helper uses.
3. Only then tackle `RS-CLIPPY-12..20`, where the current `RS-TEST-16` hits live and where the semantic-result assertions need to move into sibling assertions modules.

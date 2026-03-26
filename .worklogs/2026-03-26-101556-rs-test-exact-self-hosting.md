# RS-TEST Exact Self-Hosting

**Date:** 2026-03-26 10:15
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/test/**`

## Summary
Refactored the `rs/test` family so it obeys its own `RS-TEST-02` and `RS-TEST-03` rules without any validator carveout. The family root is now a real self-hosting test architecture root with `crates/family/runtime`, `crates/family/assertions`, and `test_support`, and the old special-case detection logic was removed from the validator.

## Context & Problem
The family had been made to pass on itself by introducing a validator exemption for a root-local "rule family implementation" shape. Manual review showed that the exemption was conceptually wrong and practically overbroad: any crate that looked `rs_*`-ish could bypass `RS-TEST-02/03`, even though the user requirement was strict self-hosting with no special status for the checker crate.

The real problem was not discovery anymore. The validator was correctly seeing the family files, but the family implementation still used a root-local single-crate layout and the validator had been taught to bless that layout. To make the self-pass legitimate, the family itself needed to adopt the same runtime/assertions architecture it enforces elsewhere.

## Decisions Made

### Remove the family exemption instead of narrowing it
- **Chose:** Delete the `is_guardrail_family_implementation_root` carveout and the `RS-TEST-02/03` branches that depended on it.
- **Why:** The user requirement is exact self-hosting. Even a narrowly scoped exemption would still encode "the checker is special," which contradicts the rule intent.
- **Alternatives considered:**
  - Narrow the heuristic to only the exact crate path — rejected because it would still be an exemption.
  - Keep the exemption and only document it better — rejected because it would preserve a false self-pass.

### Turn `families/test` into a real test-architecture root
- **Chose:** Make `apps/guardrail3/crates/app/rs/families/test` a workspace root with:
  - `crates/family/runtime`
  - `crates/family/assertions`
  - `test_support`
- **Why:** This is the closest real implementation of the `RS-TEST-03` contract without inventing a special interpretation for the family crate.
- **Alternatives considered:**
  - Keep the old top-level package and fake component manifests under it — rejected because it would split build reality from validator reality.
  - Keep a single crate and change `RS-TEST-03` to allow root-local sidecars — rejected because it would weaken the rule for normal subjects.

### Keep the runtime package name stable and rewire workspace paths
- **Chose:** Preserve the package name `guardrail3-app-rs-family-test`, move that package to `crates/family/runtime`, and update the outer workspace member/dependency paths.
- **Why:** This minimized downstream Rust import churn while still letting the family root become a self-hosting architecture root.
- **Alternatives considered:**
  - Rename the package during the move — rejected because it would force unrelated dependency and import changes.
  - Leave the outer workspace pointing at the root path — rejected because the root path is no longer the runtime crate.

### Use sibling assertions modules for rule-sidecar tests
- **Chose:** Add one assertions file per runtime rule module and rewire each sidecar test directory to import rule-local helpers from the sibling assertions crate plus generic helpers from `test_support`.
- **Why:** This satisfies the structural requirement that sidecars have a sibling assertions home and keeps tests organized around rule-local semantics instead of a monolithic in-crate helper module.
- **Alternatives considered:**
  - Keep `src/test_support.rs` inside runtime and only add empty assertions files — rejected because it would preserve the same local-helper shape that caused the self-hosting problem.
  - Put semantic helpers in `test_support` — rejected because `test_support` is meant for generic setup helpers, not rule-specific assertions.

## Architectural Notes
- The validator still discovers components only under `crates/<component>/{runtime,assertions}`. The fix here was to move the family into that shape rather than broadening the validator again.
- `test_support` is now a separate crate with generic file/project helpers plus a generic `StubToolChecker`.
- Each runtime rule-sidecar directory now re-exports only the helper surface it actually uses from its sibling assertions module and `test_support`, which keeps `-D warnings` clean.
- The old `src/` tree was moved under `crates/family/runtime/src`, and the old root-local `src/test_support.rs` was removed.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/facts.rs` — component discovery and why the family needed a real `crates/<component>` layout.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_02_owned_sidecar_shape.rs` — sidecar shape enforcement and why `mod tests;` had to become owned module names.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_03_runtime_assertions_split.rs` — proof that the validator no longer skips root-local harnesses.
- `apps/guardrail3/crates/app/rs/families/test/README.md` — prior carveout language that prompted the manual review, even though the README itself was intentionally left unchanged in this work.
- `.worklogs/2026-03-26-081002-rs-test-family-rewrite.md` — earlier family rewrite baseline.
- `.worklogs/2026-03-26-084820-validator-rs-test-self-hosting-gotcha.md` — prior note on the self-hosting blind spot.
- `.worklogs/2026-03-26-085109-rs-test-family-self-hosting.md` — earlier special-case self-hosting attempt that this change replaces.

## Open Questions / Future Considerations
- The family README still contains the now-invalid "Family Implementation Shape" carveout because earlier user instructions explicitly said not to update the README. That documentation is now stale relative to the validator and crate structure.
- Other families may eventually need the same self-hosting standard. If so, the runtime/assertions/test_support split should be applied directly rather than introducing more validator exceptions.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/Cargo.toml` — local workspace root for the family self-hosting shape.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/Cargo.toml` — live runtime crate for the family package.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/lib.rs` — family orchestrator with the exemption removed.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/facts.rs` — component/root discovery used by the family validator.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_02_owned_sidecar_shape.rs` — owned sidecar rule after removing special-case allowance.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_03_runtime_assertions_split.rs` — runtime/assertions boundary rule after removing the skip.
- `apps/guardrail3/crates/app/rs/families/test/crates/family/assertions/src/lib.rs` — one assertions module per runtime rule file.
- `apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs` — generic helper crate used by runtime sidecars and assertions.
- `apps/guardrail3/Cargo.toml` — outer workspace member rewiring for the moved family crates.
- `apps/guardrail3/crates/app/rs/Cargo.toml` — runtime dependency path update for the moved test family package.
- `.worklogs/2026-03-26-101556-rs-test-exact-self-hosting.md` — this worklog.

## Next Steps / Continuation Plan
1. Decide whether the stale carveout language in `apps/guardrail3/crates/app/rs/families/test/README.md` should now be removed, since the implementation no longer honors it.
2. Run a broader `guardrail3 rs validate apps/guardrail3 --family test --inventory --format json` sweep and distinguish real repo findings from legacy/scratch noise now that the family no longer has a self-hosting exception.
3. Apply the same "no validator exemptions, move the family instead" standard to future self-hosting family work if similar edge cases appear.

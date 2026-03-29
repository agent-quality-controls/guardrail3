# Finish RS-TEST Code Family

**Date:** 2026-03-29 15:54
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/**`

## Summary
Completed the `RS-TEST` migration for the `code` family by moving the remaining direct sidecar result proofs onto the sibling assertions surface. The family now passes its library suite, validates clean under `--family test`, and a temp-copy attack still catches reintroduced direct-hit and no-hit sidecar semantic assertions.

## Context & Problem
After `release`, `garde`, and `deps`, the `code` family became the largest remaining family-local `RS-TEST` bucket. Unlike the earlier families, `code` was already structurally split into runtime/assertions/test_support, so the problem was narrower:
- sidecars still owned semantic result proof through direct `result.id`, `result.severity`, `result.title`, `result.message`, and `result.inventory` assertions
- some false-positive tests still proved “no hit” locally with `results.len() == 0` and `result.id != "RS-CODE-.."` checks

The goal was to clear the family without weakening `RS-TEST-16` and without reopening structural loopholes that were already closed in other families.

## Decisions Made

### Convert direct sidecars to owned assertion calls instead of inventing a new helper layer
- **Chose:** rewrite each remaining direct sidecar to call existing sibling assertions helpers such as `assert_findings`, `assert_no_hits`, and `assert_files`.
- **Why:** the assertions crate already had per-rule proof surfaces and generic helper types, so the shortest honest fix was to route sidecars through that owned surface rather than create another runtime-local helper hub.
- **Alternatives considered:**
  - add a new runtime-local test helper module — rejected because that recreates the kind of crate-root tunnel `RS-TEST` is designed to forbid
  - relax `RS-TEST-16` for generic no-hit assertions — rejected because the repo direction is to make semantic ownership stricter, not softer

### Keep exact metadata checks for direct rules
- **Chose:** preserve exact title/message/file/line/inventory expectations for the converted direct sidecars instead of downgrading them to count-only checks.
- **Why:** several `code` rules are deliberately specific about emitted metadata, and the migrated tests should keep proving that exact output while moving ownership into the assertions crate.
- **Alternatives considered:**
  - reduce the direct tests to `assert_count(1)` or `assert_no_hits()` only — rejected because that would silently weaken coverage
  - move every direct expectation into new custom assertion helpers — rejected because the existing `assert_findings` surface was already adequate and much cheaper to maintain

### Treat the two attack shapes as required regressions
- **Chose:** run temp-copy attacks for both a direct-hit sidecar proof and a no-hit false-positive sidecar proof.
- **Why:** this family was almost entirely `RS-TEST-16` debt, so proving that both old shapes still trip the checker is the only convincing closeout.
- **Alternatives considered:**
  - rely only on the clean family result — rejected because a clean family without a regression attack could still mean the checker was accidentally softened elsewhere

## Architectural Notes
`code` is now a useful specimen for already-split families with dense rule inventory:
- sidecars keep scenario setup only
- sibling assertions modules own semantic proof
- false-positive “no hit” proofs also route through the owned assertions surface
- runtime did not gain any new shared test harness module

This is a different migration shape from `release`/`garde`/`deps`: the family did not need structural surgery, only semantic proof re-ownership.

## Information Sources
- Live family validation:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family test --inventory --format json`
- Family tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib`
- Code-family assertions surface:
  - `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_21_fs_glob_import.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_22_deny_forbid_without_reason.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_32_test_expect_message_quality.rs`
- Prior family worklogs:
  - `.worklogs/2026-03-29-142354-finish-rs-test-release-family.md`
  - `.worklogs/2026-03-29-152209-finish-rs-test-garde-family.md`
  - `.worklogs/2026-03-29-153735-finish-rs-test-deps-family.md`

## Open Questions / Future Considerations
- Repo-root `RS-TEST` still has other family and non-family buckets after `code`; this commit only removes the `code` family’s semantic-proof debt.
- There is unrelated dirty work in `release`, `project-tree`, and the `test` family runtime that was intentionally kept out of this checkpoint.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import_tests/direct.rs` — representative converted direct-hit sidecar
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import_tests/false_positives.rs` — representative converted no-hit sidecar
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr_tests/direct.rs` — representative rule with multiple exact message/title branches after migration
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_21_fs_glob_import.rs` — owned assertions surface used by the converted sidecars
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_22_deny_forbid_without_reason.rs` — representative multi-result assertion module
- `.worklogs/2026-03-29-153735-finish-rs-test-deps-family.md` — previous family-level `RS-TEST` migration specimen

## Next Steps / Continuation Plan
1. Commit only the `code` family sidecar changes and this worklog; leave unrelated dirty files unstaged.
2. Rerun repo-root `RS-TEST` and rank the next remaining bucket after `code` drops out.
3. Continue the sweep family-by-family with the same sequence:
   - make the target family clean under `--family test`
   - run its library tests
   - run at least one temp-copy regression that reintroduces the old sidecar-owned proof shape
   - commit with a dedicated worklog
4. Revisit the unrelated dirty `release`/`project-tree`/`test` changes separately so they do not pollute the family-level history.

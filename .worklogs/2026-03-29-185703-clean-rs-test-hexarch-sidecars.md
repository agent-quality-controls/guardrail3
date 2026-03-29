# Clean RS-TEST Hexarch Sidecars

**Date:** 2026-03-29 18:57
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/*_tests/*`

## Summary
Cleaned the entire `hexarch` family for `RS-TEST` by removing direct sidecar imports of `guardrail3_domain_report` and `guardrail3_domain_project_tree`, routing those types through the family’s owned assertions crate, and moving the last two proof-bearing result-shape checks in `rs_hexarch_01` into owned assertions helpers.

## Context & Problem
After the app-local and `test`-family cleanup commits, repo-root `RS-TEST` had collapsed to the expected family buckets. `hexarch` was the next sharpest target: `30` `RS-TEST-03` violations from sidecar imports of shared report/tree crates and `2` `RS-TEST-16` violations in `rs_hexarch_01_crates_exists_tests`.

## Decisions Made

### Re-export Shared Types From The Owned Assertions Modules
- **Chose:** Change the relevant `hexarch` assertions modules to `pub use` `CheckResult` and `Severity`, and add `ProjectTree` type aliases where the sidecar test harnesses needed tree-typed helpers.
- **Why:** The sidecars were only importing local crates for typing and proof plumbing. Moving those types behind the owned assertions surface satisfies `RS-TEST-03` without changing runtime behavior.
- **Alternatives considered:**
  - Allow direct `guardrail3_domain_report` / `guardrail3_domain_project_tree` imports in sidecars — rejected because the user explicitly said not to relax `RS-TEST`.
  - Duplicate local wrapper structs in every sidecar — rejected as needless churn and worse than re-exporting the owned types.

### Keep `rs_hexarch_01` Proof Ownership Inside Its Own Assertions Module
- **Chose:** Replace the direct result-shape assertions in `core.rs` and `ownership.rs` with owned assertions helpers, including a new helper for the rule-01/rule-12 coexistence case.
- **Why:** `RS-TEST-16` is about semantic proof ownership, not just import hygiene. Reusing the owned module keeps the sidecar limited to scenario setup and invocation.
- **Alternatives considered:**
  - Keep sidecar `assert_eq!` checks over `results` — rejected because that is the exact shape the rule bans.
  - Import the sibling `rs_hexarch_12_src_banned` assertions module — rejected because `RS-TEST-03` correctly forbids sidecars from reaching into sibling assertions modules.

### Add The Missing Assertions Dependency Explicitly
- **Chose:** Add `guardrail3-domain-project-tree` as a direct dependency of the `hexarch` assertions crate.
- **Why:** Once `ProjectTree` aliases lived in assertions, the crate needed to own that dependency instead of relying on transitive visibility from runtime.
- **Alternatives considered:**
  - Avoid typed helpers entirely and keep the `ProjectTree` references in sidecars — rejected because that would leave the `RS-TEST-03` violations in place.

## Architectural Notes
This commit reinforces the intended `RS-TEST` split: the runtime sidecars still own scenario construction and fixture mutation, but typing and semantic proof now flow through the family’s sibling assertions crate. The `ProjectTree` alias addition is especially important because tree-driven harnesses were one of the last practical excuses for leaking shared crate imports into sidecars.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/*.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/*_tests/*`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/hexarch --family test --inventory --format json`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
- repo-root `RS-TEST` snapshot in `/tmp/rs-test-root.json`
- `.worklogs/2026-03-29-184937-fix-rs-test-self-fixtures.md`

## Open Questions / Future Considerations
- Repo-root `RS-TEST` is now down to `55` errors, all `RS-TEST-03`, plus the root mutation-adoption info/warn surface.
- Remaining error buckets are `garde` and `code`; `hexarch` is no longer in the error set.
- The root `RS-TEST-14` warning and `RS-TEST-11/12/13/15` info findings remain policy questions rather than family-sidecar cleanup.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/Cargo.toml` — new explicit `ProjectTree` dependency
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_01_crates_exists.rs` — owned helper additions and type re-export pattern
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_24_cross_app_boundary.rs` — specimen for `ProjectTree` aliasing in assertions
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_01_crates_exists_tests/core.rs` — sidecar now delegates result-shape proof
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary_tests/mod.rs` — tree-driven test harness now typed through assertions
- `.worklogs/2026-03-29-184634-clean-rs-test-app-local-assertions-and-clippy.md` — prior chunk that left `hexarch` as the next major target

## Next Steps / Continuation Plan
1. Commit this `hexarch` cleanup without staging unrelated `release`, `deps`, or `Cargo.lock` changes.
2. Sweep `apps/guardrail3/crates/app/rs/families/garde` next; it is now the smaller remaining `RS-TEST-03` bucket.
3. Then sweep `apps/guardrail3/crates/app/rs/families/code`, which should be the final error bucket for `RS-TEST`.
4. After all family errors are gone, decide whether to leave the root mutation-adoption `info` findings and `RS-TEST-14` warning as-is or explicitly wire the repo hooks/config so repo-root `RS-TEST` reaches `0/0/0`.

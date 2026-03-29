# Finish RS-TEST Zero

**Date:** 2026-03-29 19:18
**Scope:** `.githooks/pre-commit`, `.plans/todo/checks/rs/test.md`, `apps/guardrail3/.githooks/pre-commit`, `apps/guardrail3/crates/app/rs/families/code/**`, `apps/guardrail3/crates/app/rs/families/garde/**`, `apps/guardrail3/crates/app/rs/families/test/**`

## Summary
Finished the repo-root `RS-TEST` sweep to `0/0/0` without weakening missing/bad-state enforcement. The work closed the last `RS-TEST-03` import-boundary bucket in `code`, finished the pending `garde` sidecar migration, and changed mutation-adoption rules to stay quiet on compliant setups while still warning on incomplete adoption.

## Context & Problem
The repo-root `RS-TEST` backlog had been reduced from the original four-digit error count to a narrow tail: direct sidecar imports in `code`, remaining local-crate imports in `garde`, and mutation-adoption rules (`RS-TEST-11` through `RS-TEST-15`) that still emitted positive inventory on healthy setups. That positive inventory made repo-root `RS-TEST` incapable of ever reaching `0/0/0`, which was inconsistent with the stabilization goal already applied to other self-hosted families.

## Decisions Made

### Code sidecars now prove through owned assertions only
- **Chose:** Re-export `CheckResult`/`Severity` from `code` assertions modules and switch the remaining `code` sidecars to use owned assertion-module imports instead of direct `guardrail3_domain_report` imports.
- **Why:** The remaining repo-root `RS-TEST-03` errors were all sidecar boundary violations, not logic defects. Re-exporting the shared report surface from owned assertions preserves the rule’s boundary intent while avoiding direct local-crate imports from sidecars.
- **Alternatives considered:**
  - Leave sidecars importing `guardrail3_domain_report` directly — rejected because it is exactly what `RS-TEST-03` forbids.
  - Relax `RS-TEST-03` for `guardrail3_domain_report` in code sidecars — rejected because that would widen the exception instead of fixing ownership.

### RS-CODE-30 test fixtures now route through runtime-owned types
- **Chose:** Re-export `DirEntry` and `ProjectTree` from `rs_code_30_input_failures.rs` under `#[cfg(test)]` and import them from `super::super` in sidecars.
- **Why:** `RS-TEST-03` bans sidecars from directly importing local production crates like `guardrail3_domain_project_tree`. A test-only runtime re-export keeps the sidecar boundary honest without moving fixture semantics into `test_support`.
- **Alternatives considered:**
  - Add `ProjectTree` builders to `code/test_support` — rejected because it would enlarge generic support with semantic runtime-owned model types.
  - Keep direct `guardrail3_domain_project_tree` imports — rejected because that was the live root error.

### Garde parity/setup helpers stay inside the family
- **Chose:** Finish the pending `garde` migration by moving remaining report-type and clippy-baseline accesses onto owned family helpers.
- **Why:** Repo-root still surfaced one `garde` `RS-TEST-03` error after the family-level cleanup. The last problem was a parity sidecar reaching `guardrail3_domain_modules::clippy::build_clippy_toml` directly.
- **Alternatives considered:**
  - Accept the parity sidecar’s direct import as a special case — rejected because it weakens the same rule the family was being migrated to satisfy.

### Mutation rules warn on incomplete adoption and stay quiet when healthy
- **Chose:** Change `RS-TEST-11` through `RS-TEST-15` so compliant mutation-adoption setups emit no inventory result, while missing/bad-state setups still warn.
- **Why:** Positive inventory on healthy mutation setups made repo-root `RS-TEST` structurally incapable of reaching `0/0/0`. Quiet compliant behavior preserves enforcement while removing noise, matching the stabilization pattern already used in `fmt`, `deny`, and other self-hosted families.
- **Alternatives considered:**
  - Keep the positive inventory and accept that repo-root `RS-TEST` can never be zero — rejected because it defeats the stabilization target.
  - Remove mutation adoption from the repo to silence the rules — rejected because that would weaken testing practice instead of fixing rule behavior.

### The app owns its mutation hook surface explicitly
- **Chose:** Add `apps/guardrail3/.githooks/pre-commit` with `cargo mutants --check`, and also wire the repo-root hook to include the same command.
- **Why:** `RS-TEST-14` discovers hook surfaces relative to the owned app root, so the existing repo-root `.githooks/pre-commit` did not satisfy `apps/guardrail3`. Adding the app-local hook made the warning disappear for the right reason.
- **Alternatives considered:**
  - Rely on the repo-root `.githooks/pre-commit` only — rejected because the rule is app-root scoped and correctly ignored it.
  - Add a dummy or documentation-only mutants mention — rejected because the rule intentionally uses executable-line matching and ignores comments/help/version-only forms.

## Architectural Notes
- `RS-TEST-03` is now clean at repo root by the intended mechanism: sidecars prove through family-owned assertions crates or runtime-owned test-only exports, not by importing unrelated local production crates.
- The mutation-adoption rules now behave like gatekeepers rather than inventory producers: adoption markers still activate the rules, and missing/bad setup still reports, but healthy mutation setups are quiet.
- The app-local hook surface under `apps/guardrail3/.githooks/pre-commit` is now part of the owned Rust app boundary for mutation adoption.

## Information Sources
- Repo-root `RS-TEST` validation output from `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family test --inventory --format json`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `.plans/todo/checks/rs/test.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs`
- Prior worklogs from this sweep, especially:
  - `.worklogs/2026-03-29-182711-finish-rs-test-hooks-rs-family.md`
  - `.worklogs/2026-03-29-184634-clean-rs-test-app-local-assertions-and-clippy.md`
  - `.worklogs/2026-03-29-184937-fix-rs-test-self-fixtures.md`
  - `.worklogs/2026-03-29-185703-clean-rs-test-hexarch-sidecars.md`

## Open Questions / Future Considerations
- The repo worktree still contains unrelated unstaged changes in `deps`, `release`, `Cargo.lock`, and `crates/domain/project-tree/src/lib.rs`. They were intentionally left out of this commit.
- `RS-CODE` still has its own large repo-root backlog; this commit only removed the `RS-TEST` boundary violations inside `code`.
- The app-local hook currently runs `cargo mutants --check`, which satisfies `RS-TEST-14` and is cheaper than a full mutation run, but the team may later decide to require a stronger hook policy in a dedicated rule/hook family.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_11_cargo_mutants_installed.rs` — mutation tool availability rule, now quiet on healthy setups
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_14_mutation_hook_present.rs` — mutation hook requirement logic
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — app-root hook discovery and executable-line matching
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_30_input_failures.rs` — test-only runtime exports for `ProjectTree` fixtures
- `apps/guardrail3/crates/app/rs/families/garde/crates/assertions/src/common.rs` — family-owned report-type re-exports for sidecars
- `apps/guardrail3/.githooks/pre-commit` — app-owned mutation hook surface satisfying `RS-TEST-14`
- `.worklogs/2026-03-29-185703-clean-rs-test-hexarch-sidecars.md` — prior checkpoint before the final repo-root sweep

## Next Steps / Continuation Plan
1. Commit this staged `RS-TEST` completion batch only; do not stage the unrelated `deps`, `release`, `Cargo.lock`, or `project-tree` worktree changes.
2. Start the next repo-root sweep with `RS-CODE`, using the now-clean `RS-TEST` results so code-family findings are not mixed with sidecar-boundary debt.
3. When `RS-CODE` is done, re-run repo-root validation for `test`, `code`, `release`, and `deny` together to confirm no regressions leaked across the family boundaries.

# Clean RS-TEST Arch And Toolchain Follow-Ups

**Date:** 2026-03-29 16:49
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/*`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/assertions/src/*`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_*_tests/mod.rs`

## Summary
Closed the remaining repo-root `RS-TEST` follow-ups in `arch` and `toolchain`. `arch` only needed a stale assertions-crate dependency removal. `toolchain` needed its runtime sidecars to stop importing `guardrail3_domain_report` directly and to push the last family-level semantic result checks back into the owned `rs_toolchain_01_exists` assertions module.

## Context & Problem
After the earlier family-by-family `RS-TEST` sweep, repo-root validation still showed two small leftovers even though the families were mostly clean in isolation:
- `arch` still emitted one repo-root `RS-TEST-03` hit from `crates/assertions/src/lib.rs` because the assertions crate declared and imported `guardrail3_domain_project_tree` even though it no longer used it.
- `toolchain` had four repo-root `RS-TEST-03` hits from sidecars importing `guardrail3_domain_report::Severity`, plus three family-local `RS-TEST-16` hits in `rs_toolchain_01_exists_tests/mod.rs` where the sidecar still asserted cross-rule family results directly.

The goal was to remove those residual errors without weakening `RS-TEST`, and without touching the unrelated dirty `release`, `clippy`, `Cargo.lock`, or `project-tree` work already in the tree.

## Decisions Made

### Remove the stale `arch` assertions dependency instead of masking it
- **Chose:** delete the unused `guardrail3_domain_project_tree` dependency from `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml` and keep the crate root imports minimal.
- **Why:** the import was only there to satisfy an older lint workaround, but it now produced a real repo-root `RS-TEST-03` violation with no architectural value.
- **Alternatives considered:**
  - Keep the dependency and try to exempt assertions crate roots from `RS-TEST-03` — rejected because it would reopen an unnecessary local-crate tunnel.
  - Keep the dependency and reference it indirectly — rejected because there was no legitimate use left.

### Keep `toolchain` sidecars inside their owned assertions boundary
- **Chose:** re-export `Severity` from the `toolchain` assertions modules and replace the remaining `guardrail3_domain_report::Severity` sidecar imports with those owned re-exports.
- **Why:** the repo-root errors were boundary errors, not missing coverage. The sidecars already had the right assertions modules; they just needed to stop reaching outside the family-owned surface.
- **Alternatives considered:**
  - Allow direct `guardrail3_domain_report` imports in sidecars — rejected because that is exactly what `RS-TEST-03` is forbidding.
  - Move severity expectations into `test_support` — rejected because severity/result semantics belong in the assertions crate, not generic support.

### Move cross-rule family result proof into the owned `rs_toolchain_01_exists` assertions module
- **Chose:** add owned helper functions in `rs_toolchain_01_exists` for the legacy-only, malformed-modern-plus-legacy, and invalid-root-cargo family scenarios, then rewrite the sidecar to call those helpers.
- **Why:** the first attempt at using sibling assertions modules (`rs_toolchain_02`, `03`, `04`) proved the boundary is stricter than that: sidecars may only import their owned assertions module. The correct home for these cross-rule family assertions is therefore the rule-01 owned assertions module.
- **Alternatives considered:**
  - Keep the direct `results.iter().any(...)` checks in the sidecar — rejected because that is the `RS-TEST-16` shape we were fixing.
  - Import sibling assertions modules from the sidecar — rejected because family validation immediately flagged that as `RS-TEST-03`.

## Architectural Notes
This follow-up reinforces two points from the broader `RS-TEST` migration:
- repo-root validation is stricter than isolated family runs because it catches direct local-crate imports that family-local paths can miss
- sidecars do not get to aggregate proof across sibling assertions modules; if a rule’s tests need cross-rule family exactness, the owned assertions module for that rule has to expose that proof surface itself

`toolchain` is now a good small specimen of that pattern: the sidecar owns scenario setup, while the owning assertions module can still prove multi-rule family outcomes when the scenario logically belongs to that rule.

## Information Sources
- Live repo-root validation:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family test --inventory --format json`
- Family-local validation:
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/toolchain --family test --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/arch --family test --inventory --format json`
- Family tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-toolchain --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-arch --lib`
- Prior worklogs:
  - `.worklogs/2026-03-29-161458-finish-rs-test-arch-family.md`
  - `.worklogs/2026-03-29-160952-finish-rs-test-deny-family.md`

## Open Questions / Future Considerations
- Repo-root `RS-TEST` is still dominated by the legacy `hooks-shared` / `hooks-rs` structure and by direct sidecar-local imports in `clippy`, plus a few scattered non-family flat sidecars.
- There are unrelated dirty edits in `clippy`, `release`, `Cargo.lock`, and `crates/domain/project-tree/src/lib.rs`; they were intentionally excluded from this checkpoint and still need their own lane.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/assertions/src/rs_toolchain_01_exists.rs` — now owns the cross-rule family proof helpers that the sidecar is allowed to call
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_01_exists_tests/mod.rs` — representative migrated sidecar staying within its owned assertions boundary
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/assertions/src/rs_toolchain_config_01_channel_components.rs` — representative `Severity` re-export used to remove sidecar-local `domain_report` imports
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml` — cleaned assertions-crate dependency boundary for `arch`
- `.worklogs/2026-03-29-161458-finish-rs-test-arch-family.md` — immediate background on the earlier `arch` family cleanup

## Next Steps / Continuation Plan
1. Commit only the `arch` and `toolchain` follow-up files with this worklog; keep the dirty `clippy`, `release`, `Cargo.lock`, and `project-tree` files out of the commit.
2. Refresh repo-root `RS-TEST` and keep using it as the primary backlog source; after this checkpoint it should be mostly `hooks-shared`, `hooks-rs`, and `clippy`.
3. Finish the direct-import `clippy` sidecars next if a quick family-local sweep can remove a large chunk without colliding with the dirty `release` lane.
4. After `clippy`, take the structurally heavier legacy hooks families, likely starting with `hooks-rs` before the larger `hooks-shared` migration.

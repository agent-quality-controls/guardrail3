# Compile Frontier Cleanup Checkpoint

**Date:** 2026-03-30 16:51
**Scope:** `apps/guardrail3/crates/adapters/outbound/report/*`, `apps/guardrail3/crates/app/arch-helpers/src/lib.rs`, multiple Rust family runtime files under `apps/guardrail3/crates/app/rs/families/*/src`, multiple Rust family assertion crates under `apps/guardrail3/crates/app/rs/families/*/crates/assertions/src`, Rust family test-support crates, and legacy TS ESLint validation files under `apps/guardrail3/crates/app/ts/validate/eslint/*`

## Summary
This batch pushes the repo well past the previous compile-collapse frontier by repairing a large number of malformed Rust rule files, normalizing many assertion crates to the current `CheckResult` accessor API, and updating test-support helpers to the current private `ProjectTree` / `DirEntry` constructor model. The workspace is not fully green yet, but the remaining frontier is much smaller and mostly isolated to a handful of malformed files plus some assertion expectation type fixes.

## Context & Problem
After the runtime/family split work, the repo still had a very large compile break surface. The immediate failure mode was not architectural; it was repo-wide code drift caused by partially applied mass rewrites:

- a broad codemod had moved `CheckResult` consumers from field access toward methods, but many files were left with corrupted repeated call chains such as `result.id()()()()`
- a separate bad rewrite damaged many `CheckResult::from_parts(...)` call sites, leaving mismatched delimiters like `});`, `)`, or `.as_inventory()` wrapped around malformed expressions
- test-support crates still constructed `ProjectTree` and `DirEntry` directly even though those structs now use private fields with constructor APIs

The result was thousands of noisy compile errors that obscured the true remaining blockers. The goal of this checkpoint was to collapse that noise into a smaller, honest frontier that can be resumed from a later session.

## Decisions Made

### Normalize assertion crates broadly instead of fixing them one family at a time
- **Chose:** apply repeated mechanical cleanup across assertion crates to move them onto the accessor API (`id()`, `severity()`, `title()`, `message()`, `file()`, `line()`, `inventory()`)
- **Why:** the failures were highly repetitive and spread across many family assertion crates; fixing them only where the compiler first stopped would have wasted time and hidden the real remaining frontier
- **Alternatives considered:**
  - Fix only the currently failing family crates — rejected because the compiler kept surfacing the same codemod damage family by family
  - Revert all assertion crates to field access — rejected because the project has already moved toward accessor APIs and private-field boundaries

### Repair malformed production files directly instead of trying to infer intent through more codemods
- **Chose:** hand-fix and in several cases fully rewrite broken rule/helper files where delimiters or helper endings were clearly corrupted
- **Why:** once a rule file had unbalanced delimiters, extra `);`, or broken helper trailers, another mechanical pass risked making the syntax damage worse
- **Alternatives considered:**
  - Use another repo-wide delimiter rewrite — rejected because the earlier bad rewrite was the source of much of the damage
  - Leave malformed files until assertions were fully clean — rejected because syntax errors stop the compiler before later API cleanup can even be evaluated

### Update test-support helpers to constructor APIs
- **Chose:** switch helper crates from direct `ProjectTree` / `DirEntry` field construction to `ProjectTree::new(...)` and `DirEntry::new(...)`
- **Why:** these are real API changes, not transient syntax issues; continuing to use direct struct literals would keep the repo pinned to a removed public-field contract
- **Alternatives considered:**
  - Make the structs public again — rejected because that would undo the intended encapsulation change
  - Ignore test-support until runtime is green — rejected because several family crates compile their support crates as part of the package graph

## Architectural Notes
This work does not change the architecture. It is a cleanup batch on top of the current split runtime/family design.

Important practical findings:

- The largest compile blast radius came from assertion crates, not production family logic.
- The syntax damage was consistent with a bad automated rewrite, not independent hand mistakes in every file.
- The accessor migration and the delimiter corruption were separate problems that overlapped in the same files.
- The repo is now much closer to a truthful compiler frontier: instead of thousands of repeated accessor failures, the main remaining blockers are a smaller set of malformed runtime files plus some assertion expectation type mismatches.

## Information Sources
- Current compiler frontier from repeated runs of:
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet`
- Current `CheckResult` API in:
  - `apps/guardrail3/crates/domain/report/mod.rs`
- Current `ProjectTree` / `DirEntry` constructor APIs in:
  - `apps/guardrail3/crates/domain/project-tree/src/lib.rs`
- Existing repo state and prior context from:
  - `.worklogs/2026-03-30-152626-rs-migration-batch.md`
  - `.worklogs/2026-03-30-152626-plans-and-handoffs-refresh.md`
  - `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md`

## Open Questions / Future Considerations
- Legacy TS validation files under `apps/guardrail3/crates/app/ts/validate/eslint/` still have malformed syntax from the same rewrite pattern. The active product direction is Rust-only, but these files still compile as workspace members and therefore still block the full workspace.
- Some assertion crates, especially `release` and `clippy`, still need type-shape cleanup where expected values are now `String`/borrowed `&str` mismatches after the accessor migration.
- A few remaining runtime files still have malformed delimiters and should be handled as direct rewrites, not more codemods.

## Key Files for Context
- `apps/guardrail3/crates/domain/report/mod.rs` — authoritative `CheckResult` API; many assertion edits depend on this contract
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — authoritative `ProjectTree` / `DirEntry` constructors and private-field boundary
- `apps/guardrail3/crates/app/arch-helpers/src/lib.rs` — repaired shared structural helper crate that had severe delimiter corruption
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_18_test_support_generic.rs` — example of remaining malformed runtime-file frontier
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_10_test_uses_workspace.rs` — example of remaining malformed hook-family frontier
- `apps/guardrail3/crates/app/rs/families/release/assertions/src/common.rs` — example of remaining assertion type-shape frontier
- `apps/guardrail3/crates/app/ts/validate/eslint/eslint_check.rs` — remaining legacy TS syntax frontier that still blocks full workspace compile
- `.worklogs/2026-03-30-152626-rs-migration-batch.md` — prior committed migration checkpoint
- `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md` — prior family-split verification context

## Next Steps / Continuation Plan
1. Repair the remaining malformed runtime files directly:
   - `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_config_20_skip_hygiene.rs`
   - `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_10_test_uses_workspace.rs`
   - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_18_test_support_generic.rs`
   - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_33_public_weak_error_forms.rs`
2. Decide whether legacy TS ESLint validation should still be kept build-green. If yes, repair `apps/guardrail3/crates/app/ts/validate/eslint/eslint_check.rs`; if no, remove it from the active product build path.
3. Finish assertion cleanup in the remaining crates that now fail on type mismatches rather than syntax:
   - `release` assertions
   - `clippy` assertions
4. Re-run `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet` after each batch until the frontier stops moving.
5. Once compile-green, run targeted package checks on the most-edited families before making any broader claims about repo health.

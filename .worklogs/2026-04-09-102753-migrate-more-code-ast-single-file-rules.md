# Migrate More Code AST Single-File Rules

**Date:** 2026-04-09 10:27
**Scope:** `packages/rs/code/g3rs-code-ast-checks`, `packages/rs/code/g3rs-code-ast-ingestion`, `.plans/todo/checks/rs/code.md`

## Summary
Migrated five more clearly single-file `code` rules into the AST package: too many effective code lines, too many `use` imports, many `use` imports warning threshold, large type inventory, and `#[path = "..."]` with reason checks. Added the parse helpers they needed, added rule-local tests, extended the pipeline test, and updated the ledger wording for the effective-line rule.

## Context & Problem
After the previous migration batch, the remaining low-risk single-file `code` rules were the structural-per-file checks:

- too many effective code lines
- too many `use` imports
- large struct/enum inventory
- `#[path = "..."]` module redirection

These still fit the same AST lane:
- one Rust file in
- AST plus raw source text
- no workspace-wide graph needed

The only wrinkle was `RS-CODE-24`: the old app code had later moved it to `RS-ARCH-09`, while the current `code` ledger still keeps it in the `code` family. For the package migration, I followed the current `code` ledger and the documented `RS-CODE-24` contract instead of the later arch-only simplification.

## Decisions Made

### Keep “too many effective code lines” as a normal rule, not ingestion failure
- **Chose:** Treat the file-length policy as a normal AST rule result.
- **Why:** The file is still readable and parseable. The problem is policy size, not inability to ingest.
- **Alternatives considered:**
  - Fail ingestion on long files — rejected because that would hide a normal policy violation behind an input failure.

### Use the clearer rule language in the migrated rule and ledger
- **Chose:** Name `RS-CODE-09` as “too many effective code lines”.
- **Why:** The user explicitly preferred that wording over “file too long”.
- **Alternatives considered:**
  - Keep the old title text — rejected because it is vaguer and less accurate.

### Keep `RS-CODE-24` in the code AST package for now
- **Chose:** Implement `#[path = "..."]` checks in `g3rs-code-ast-checks`.
- **Why:** The current rule ledger still places it in `RS-CODE`, and the rule is still single-file AST + source-text.
- **Alternatives considered:**
  - Skip it because legacy app moved it to `arch` — rejected because that would leave the current `code` contract unmigrated.
  - Copy the newer `RS-ARCH-09` behavior exactly — rejected because that newer rule is a stricter “always forbidden” policy, while the `code` ledger still specifies reason handling and the test-sidecar exemption.

### Reuse the same parse/support layer instead of inventing new plumbing
- **Chose:** Extend the existing AST runtime helpers with:
  - effective code-line counting
  - top-level `use` import counting
  - large type discovery
  - `#[path]` attribute discovery
- **Why:** These are still single-file parse-time facts and belong in the checks runtime.
- **Alternatives considered:**
  - Push any of this into ingestion — rejected because ingestion should stay at file selection / file reading / mapping.

## Architectural Notes
- `parse/comments` now again owns effective non-comment line counting because that is a source-text property, not an AST property.
- `parse/core` now owns recursive `use`-tree counting.
- `parse/visitors` now owns large struct/enum inventory discovery.
- `parse/attrs` now owns `#[path = "..."]` detection, including:
  - direct `#[path]`
  - `cfg_attr(..., path = "...")`
  - real parent-segment detection via path segments instead of raw substring matching
  - test-sidecar exemption for `rule_tests/mod.rs` and `*_tests/mod.rs`
- The public ingestion boundary did not change. `g3rs-code-ast-ingestion` still just selects files and hands source content to the AST package.

## Information Sources
- `.plans/todo/checks/rs/code.md` — current `code` rule ledger and the stated `RS-CODE-24` contract.
- Legacy runtime rules:
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_09_file_length/rule.rs`
  - `.../rs_code_10_use_count_error/rule.rs`
  - `.../rs_code_11_use_count_warn/rule.rs`
  - `.../inventory/rs_code_19_large_type_inventory/rule.rs`
- Legacy parse helpers:
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments/mod.rs`
  - `.../parse/core.rs`
  - `.../parse/visitors/mod.rs`
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — especially the `RS-CODE-24` path-semantics note and the line-count naming/semantics note.
- `.worklogs/2026-04-09-095636-migrate-code-ast-reason-comment-rules.md` — prior AST migration batch that established the current package/test pattern.

## Open Questions / Future Considerations
- `RS-CODE-24` now exists again in the package lane, but the legacy app later re-owned a stricter version under `RS-ARCH-09`. That family-boundary conflict should be resolved explicitly later.
- The current `RS-CODE-24` exemption logic intentionally supports both `rule_tests/mod.rs` and `*_tests/mod.rs` because both patterns exist in migration discussions. If one canonical shape wins later, narrow the exemption.

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/run.rs` — full list of currently migrated code AST rules.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/comments/mod.rs` — effective code-line counting and same-line comment logic.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/core.rs` — top-level `use` import counting.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — `#[path]`, lint-policy, and cfg-attr attribute analysis.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/visitors/mod.rs` — large type inventory detection.
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end AST lane coverage for the migrated rules.
- `.plans/todo/checks/rs/code.md` — ledger for what still remains in `code`.
- `.worklogs/2026-04-09-095636-migrate-code-ast-reason-comment-rules.md` — previous migration batch and related rule-boundary decisions.

## Next Steps / Continuation Plan
1. Audit the remaining unmigrated `code` rules again and separate them into:
   - still-single-file and ready now
   - single-file but blocked on `profile_name`
   - not this AST lane
2. Migrate the remaining still-single-file rules that do not need profile resolution.
3. Then add real `profile_name` resolution in `g3rs-code-ast-ingestion` so the library-profile rules can move cleanly.
4. Once the next batch lands, run another attack pass focused on the remaining `code` family overlaps, especially `RS-CODE-24` versus `RS-ARCH-09`.

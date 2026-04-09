# Migrate code AST API-shape rules

**Date:** 2026-04-09 13:30
**Scope:** `packages/rs/code/g3rs-code-ast-checks/crates/assertions/src/lib.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/assertions/src/rs_code_29_large_trait_surface.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/assertions/src/rs_code_31_public_struct_named_fields.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/assertions/src/rs_code_33_public_weak_error_forms.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/lib.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/run.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/mod.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/types.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/analysis_helpers.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/visitors/mod.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_29_large_trait_surface/**`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/**`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/**`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Summary
Migrated the three remaining live single-file API-shape rules from the `code` family into `g3rs-code-ast-checks`: large trait surface, public struct field bags, and weak public error forms. The work reused the existing single-file AST lane, pulled over the minimal parse helpers those rules needed, and extended the end-to-end pipeline test so the new rules are exercised through crawl -> ingestion -> checks.

## Context & Problem
After profile resolution landed, the next step was to decide which `code` rules were still live enough to migrate. The old ledger still listed `RS-CODE-25`, `26`, and `27`, but the legacy runtime already treats them as dead or moved:
- `RS-CODE-25` is an intentional non-firing placeholder
- `RS-CODE-26` and `RS-CODE-27` were retired as redundant with architecture rules

That meant the remaining real AST work in `code` was:
- `RS-CODE-29` large trait surface
- `RS-CODE-31` public structs with named `pub` fields
- `RS-CODE-33` weak public error forms

The extracted package did not yet expose the parse helpers for those three rules, so the migration needed a small parse-layer expansion before wiring the rules themselves.

## Decisions Made

### Migrated only the still-firing rules
- **Chose:** Port `RS-CODE-29`, `31`, and `33`, and leave `25`, `26`, and `27` alone.
- **Why:** The live legacy runtime already documents `25` as intentionally silent and `26`/`27` as removed ownership. Reviving them in the new package would create divergence from current behavior.
- **Alternatives considered:**
  - Port everything still listed in the old ledger — rejected because the old ledger is stale for these three ids.
  - Skip `29` until profile-gating semantics are clearer — rejected because the live rule still fires directly today, and the extracted package should match that behavior.

### Expanded the parse layer only where required
- **Chose:** Add the smallest missing AST helpers for:
  - large trait counting
  - public field bag detection
  - weak public `Result` error detection
- **Why:** Those were the only missing primitives needed for the next three rules.
- **Alternatives considered:**
  - Copy the old parse module wholesale — rejected because it would drag in dead surfaces and make the extracted package harder to audit.
  - Re-implement each rule by walking raw `syn` directly in the rule files — rejected because it would break the established split where the parse/support layer owns AST extraction and rules stay small.

### Kept the new rules aligned to live runtime behavior, not stale doc wording
- **Chose:** Preserve the legacy firing behavior exactly:
  - `RS-CODE-29` warns above 8 methods and errors above 12
  - `RS-CODE-31` warns for 1-4 public named fields and errors at 5+
  - `RS-CODE-33` owns all weak public error forms across `String`, `&str`, `anyhow::Error`, and `Box<dyn Error>`
- **Why:** The user asked for actual workable behavior, and the live runtime is the better source than stale ledger notes here.
- **Alternatives considered:**
  - Re-interpret these rules as library-only up front — rejected because the current live rule code does not enforce that boundary.

## Architectural Notes
The package shape stayed consistent:
- one rule per folder
- local helper-based rule tests
- assertions crate exports per rule
- runtime parse/support layer owns AST discovery

The `code` AST lane now covers all still-live single-file source rules that belong in this family package. What remains in the old `code` ledger is either:
- not AST (`RS-CODE-07`, `RS-CODE-12`, `RS-CODE-35`)
- intentionally dead (`RS-CODE-25`)
- moved ownership (`RS-CODE-26`, `RS-CODE-27`)

The end-to-end ingestion pipeline test was extended to prove the three new rules on ingested files, not just unit-test helpers.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_25_*` — confirmed `RS-CODE-25` is intentionally silent
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_26_*` — confirmed `RS-CODE-26` was removed as redundant with architecture checks
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_27_*` — confirmed `RS-CODE-27` was removed as redundant with architecture checks
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/inventory/rs_code_29_large_trait_inventory/*`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_31_public_struct_named_fields/*`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_33_public_weak_error_forms/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/*` — extracted parse layer expanded in this change
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end coverage for the new rules

## Open Questions / Future Considerations
- The rule ledger still contains stale entries for `25`, `26`, and `27`. The next doc cleanup should make that explicit so future sessions do not try to re-port dead rule surfaces.
- If `29`, `31`, or `33` should become profile-gated later, make that an explicit policy change with tests, not an accidental side effect of the new profile context now being available.

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_29_large_trait_surface/rule.rs` — extracted large-trait rule
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs` — extracted public field bag rule
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_33_public_weak_error_forms/rule.rs` — extracted weak public error rule
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — public API shape AST extraction helpers
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/visitors/mod.rs` — large trait visitor
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — pipeline proof for the newly migrated rules
- `.worklogs/2026-04-09-131549-implement-code-ast-profile-resolution.md` — previous step that added profile context to the AST lane

## Next Steps / Continuation Plan
1. Clean up the `code` ledger so `25`, `26`, and `27` are no longer presented like live migration targets.
2. Decide whether any of `29`, `31`, or `33` should actually become profile-gated in the package world, and only change that if there is explicit policy backing.
3. Shift the next migration effort away from `code` AST and toward the non-AST leftovers:
   - `RS-CODE-07` config/file text lane
   - `RS-CODE-12` config lane
   - `RS-CODE-35` file-tree lane

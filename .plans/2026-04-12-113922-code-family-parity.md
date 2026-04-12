# Code Family Parity

## Goal

Bring the `code` family to an honest end state by reconciling the live app family with the extracted package family. The immediate target is to resolve the remaining app-only tail, remove dead-rule drift, and settle the `RS-CODE-SOURCE-24` vs `arch` ownership conflict from live code instead of comments.

## Current live gap

App `code` still executes these rule IDs that are not cleanly represented in the packages:

- `RS-CODE-25` - dead stub, explicitly superseded by weak error-form handling
- `RS-CODE-26` - dead stub, explicitly marked redundant with `arch`
- `RS-CODE-27` - dead stub, explicitly marked redundant with `arch`
- `RS-CODE-35` - live structural-cap rule, still app-only

There is also a live ownership conflict:

- app code says `RS-CODE-24` moved to `RS-ARCH-09`
- package code still implements `RS-CODE-SOURCE-24`
- the package implementation is not the same thing as `RS-ARCH-09`
  - `RS-CODE-SOURCE-24` checks `#[path]` usage plus same-line `// reason:` quality and parent-directory escape
  - `RS-ARCH-SOURCE-09` is the broader "no `#[path = ...]`" rule

## Approach

1. Re-audit the old app `code` rule bodies against the package rules and classify each remaining delta as:
   - dead legacy stub to remove from app
   - real remaining package migration work
   - cross-family ownership conflict to resolve
2. Resolve `RS-CODE-24` ownership deliberately.
   - Read the live `arch` and `code` rule bodies side by side.
   - Decide whether package `RS-CODE-SOURCE-24` stays in `code` or is retired because `arch` fully subsumes it.
   - Update package docs, app docs, and app runtime comments to match the real decision.
3. Migrate `RS-CODE-35` into a package lane if it is still a live rule.
   - This is structural, so it belongs in a `code` file-tree lane if the rule survives.
   - Build only the minimal new package surface needed for that rule.
4. Delete or retire dead app stubs `RS-CODE-25`, `26`, and `27` from the old app path once tests prove they are non-firing and redundant.
5. Add parity tests that prove the final app/package rule distribution instead of relying on README text.

## Key decisions

- Do not trust the old app comment that `RS-CODE-24` moved to `arch` until the rule bodies are compared directly.
- Treat `RS-CODE-25`, `26`, and `27` as cleanup candidates, not automatic migration candidates.
- Only build a `code` file-tree package if `RS-CODE-35` remains a real rule after the audit.
- Keep this scoped to the live `code` family. Do not batch `garde` or other families into the same change.

## Files to read/modify

- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/run.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_25_public_result_error_type/rule.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_26_lib_glob_reexport/rule.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/api_shape/rs_code_27_facade_only_lib/rule.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/inventory/rs_code_35_root_structural_cap/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/code/g3rs-code-source-checks/README.md`
- `packages/rs/code/g3rs-code-source-checks/TODO.md`
- `packages/rs/code/g3rs-code-ingestion/TODO.md`

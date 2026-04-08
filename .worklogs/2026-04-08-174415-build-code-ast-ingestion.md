# Build Code AST Ingestion

**Date:** 2026-04-08 17:44
**Scope:** `packages/rs/code/g3rs-code-ast-ingestion`, `packages/rs/code/g3rs-code-ast-checks`

## Summary
Built `g3rs-code-ast-ingestion` as the first dedicated AST ingestion package. It now selects Rust source files from `g3rs-workspace-crawl`, skips fixture paths, classifies test-owned files, reads source contents, and emits `G3RsCodeAstChecksInput`.

## Context & Problem
The first AST checks package, `g3rs-code-ast-checks`, already existed, but there was still no package that could produce its public input from the workspace crawl. The agreed architecture for AST is that ingestion owns scope selection and file reading, while the AST checks runtime owns parsing and semantic mapping.

## Decisions Made

### Build the first AST ingestion package around the `code` family
- **Chose:** `packages/rs/code/g3rs-code-ast-ingestion`
- **Why:** `code` is the clean single-file AST specimen, so it is the least risky place to prove the package split
- **Alternatives considered:**
  - start with `garde` — rejected because multi-file scope would add cross-file complexity before the ingestion pattern itself was proven
  - keep using app-side discovery only — rejected because the point of the new package pipeline is to move this mapping into packages

### Keep the first package narrow
- **Chose:** select `.rs` files, skip fixtures, classify `is_test`, read file text, and emit one checks input per file
- **Why:** this is enough to drive the already-migrated `RS-CODE-13`, `RS-CODE-15`, and `RS-CODE-16` rules end to end
- **Alternatives considered:**
  - resolve `profile_name` now — rejected because current migrated rules do not need it and the policy routing would drag in extra design work
  - parse AST during ingestion — rejected because AST parsing belongs in the checks runtime

### Leave `profile_name` unset for now
- **Chose:** populate `profile_name` as `None`
- **Why:** the public checks input already has room for it, but the first migrated rule slice does not consume it
- **Alternatives considered:**
  - invent a partial policy resolver here — rejected because that would guess at a policy contract we have not locked yet

## Architectural Notes
This package proves the AST lane split in the intended direction:

- crawl is neutral
- AST ingestion chooses files and reads bytes
- AST checks runtime parses and maps
- rules stay tiny and pure

The runtime includes a smoke test that runs ingested files through `g3rs-code-ast-checks`, so the package contract is exercised, not just the selection logic.

## Information Sources
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md`
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`
- `packages/rs/code/g3rs-code-ast-checks`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/discover.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts/mod.rs`
- `.worklogs/2026-04-08-171620-code-ast-checks-initial-slice.md`

## Open Questions / Future Considerations
- `profile_name` still needs a real policy resolver
- fixture filtering may deserve a shared helper if more AST families need the same rule
- future `code` AST rules may require richer non-source context; that should still enter through ingestion, not filesystem reads inside checks

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/run.rs` — public ingestion entry point
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/select.rs` — source file selection and metadata shaping
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/classify.rs` — fixture and test-path classification
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/basic.rs` — package-level behavior and smoke coverage
- `packages/rs/code/g3rs-code-ast-checks/crates/types/src/lib.rs` — public AST checks input contract
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md` — AST lane template
- `.worklogs/2026-04-08-171620-code-ast-checks-initial-slice.md` — backstory for the matching checks package

## Next Steps / Continuation Plan
1. Resolve `profile_name` in `g3rs-code-ast-ingestion` once the code-family policy source is locked for packages.
2. Migrate the next batch of single-file `code` AST rules into `g3rs-code-ast-checks`.
3. After `code` grows a bit more, use the same ingestion/checks split for the first bounded multi-file AST family.

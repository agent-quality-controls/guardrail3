# code AST Checks Initial Slice

**Date:** 2026-04-08 17:16
**Scope:** `packages/rs/code/g3rs-code-ast-checks`

## Summary
Created the first real AST checks package specimen: `g3rs-code-ast-checks`. This initial slice is intentionally small and single-file only. It includes the package scaffold, public AST input types, a runtime-local parsed source shape, and the first three migrated `code` rules: `RS-CODE-13`, `RS-CODE-15`, and `RS-CODE-16`.

## Context & Problem
The repo needed one concrete AST package to prove the architecture note could become real code. The `code` family was chosen because its source-rule slice is single-file and avoids cross-file semantic mapping. Starting with `garde` or `test` would have forced multi-file AST semantics before the base package pattern was proven.

The user also asked how parity would be proven without hand-waving. For this first slice, the answer is rule-by-rule parity against the existing old-family tests rather than one giant family golden fixture.

## Decisions Made

### Start with a narrow `code` AST slice
- **Chose:** Build `g3rs-code-ast-checks` first, but only for the clearly single-file AST rules `13`, `15`, and `16`.
- **Why:** This proves the AST package boundary without mixing in workspace/config/root concerns from the broader `code` family.
- **Alternatives considered:**
  - Start with the whole `code` family — rejected because `code` overall is mixed-scope, not purely single-file.
  - Start with `garde` — rejected because it needs bounded multi-file AST support and is a worse first specimen.

### Keep the public AST input text-based
- **Chose:** The public input is `G3RsCodeAstChecksInput { source_file }`, where `source_file` carries `rel_path`, `content`, `is_test`, and optional `profile_name`.
- **Why:** This follows the AST architecture note: ingestion should read files, while the checks runtime parses and maps from bounded input.
- **Alternatives considered:**
  - Public input as file paths — rejected because that would let the checks package reach back into the filesystem.

### Use a small runtime-local parsed source container
- **Chose:** Keep `G3RsCodeSourceFileAst { source_file, ast }` inside the runtime/support layer only.
- **Why:** It keeps the parsed AST attached to the exact source input without turning the public package contract into a parsed-AST contract.
- **Alternatives considered:**
  - No parsed container at all — rejected because helper/rule wiring gets noisy quickly.
  - Make the parsed container public — rejected because the package boundary should stay source-text based.

### Prove parity rule-by-rule first
- **Chose:** Port the old direct tests for the first three rules into the new package runtime tests.
- **Why:** This is the fastest honest parity check for the first slice. The old `code` family already has dense rule-local tests, so they are the natural baseline.
- **Alternatives considered:**
  - Build one big family golden fixture first — rejected for this step because ingestion does not exist yet and the first goal is package-rule parity, not end-to-end family parity.
  - Trust manual inspection of copied logic — rejected because it would not prove the package behavior matches the old rule behavior.

## Architectural Notes
The new package follows the standard split:

- facade crate
- `crates/types`
- `crates/assertions`
- `crates/runtime`

The runtime currently owns:

- parsing one source file once
- local source-rule fan-out
- the minimal parse helpers needed by the first three rules

This first slice deliberately avoids:

- ingestion
- root/workspace discovery
- root-level lint checks like `RS-CODE-12`
- structural aggregate checks like `RS-CODE-35`
- cross-file AST semantics

## Information Sources
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md`
- `.plans/todo/checks/rs/code.md`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_13_todo_macros/**`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_15_direct_fs_usage/**`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_16_panic_macro/**`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/**`
- `packages/rs/garde/g3rs-garde-ast-checks/**` as the scaffold specimen only, not as architecture authority

## Open Questions / Future Considerations
- The next step is `g3rs-code-ast-ingestion`, which will finally prove the public text-based AST input through real crawl selection.
- A package-level parity smoke test comparing old family outputs and new package outputs on a shared fixture set is still useful, but it belongs after ingestion exists.
- More `code` AST rules can move in waves once the package boundary is stable.

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/types/src/lib.rs` — public AST input contract
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/support.rs` — runtime-local parsed source shape
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/run.rs` — AST package orchestrator
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/**` — minimal copied AST/source helpers
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_13_todo_macros/**` — first migrated rule
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/**` — first migrated rule
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_16_panic_macro/**` — first migrated rule
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md` — AST package template used here

## Next Steps / Continuation Plan
1. Build `g3rs-code-ast-ingestion` so the new package can be driven from `g3rs-workspace-crawl`.
2. Add a small package-level parity smoke test once ingestion exists, comparing the migrated rule outputs on shared source fixtures.
3. Move the next `code` single-file AST rules in small batches, keeping parity rule-by-rule.
4. Only after the single-file pattern is stable, start the first bounded multi-file AST package (`garde`).

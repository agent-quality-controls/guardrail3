## Goal

Build the real `g3rs-garde` AST lane end to end so `crawl -> ingest_for_ast_checks -> g3rs-garde-ast-checks::check` works, and preserve fail-closed `RS-GARDE-10` behavior for owned unreadable or malformed inputs.

## Approach

1. Prove the current extracted lane gap with tests.
   - Add ingestion tests that expect real `ingest_for_ast_checks(...)` output.
   - Add AST package tests that prove unreadable and malformed source currently fail open.

2. Fix the AST package boundary first.
   - Extend `g3rs-garde-ast-checks` so it can emit `RS-GARDE-10` for:
     - unreadable Rust source files
     - malformed Rust source files
     - unreadable `guardrail3.toml`
     - malformed `guardrail3.toml`
   - Keep parsing in the AST checks runtime, not in ingestion.

3. Implement root-scoped `g3rs-garde-ingestion::ingest_for_ast_checks(...)`.
   - Select owned Rust source files from the crawl.
   - Skip fixture paths.
   - Require root `guardrail3.toml`.
   - Build one `G3RsGardeAstChecksInput` for the crawled root.

4. Add end-to-end pipeline tests.
   - `crawl -> ingest_for_ast_checks -> check`
   - good root
   - malformed source reaching `RS-GARDE-10`
   - malformed `guardrail3.toml` reaching `RS-GARDE-10`

## Key decisions

- Keep AST ingestion as file gathering only.
  - Rejected reparsing in ingestion because it would make `RS-GARDE-10` unreachable through the real lane.

- Keep `guardrail3.toml` in the AST input.
  - The extracted AST package already uses it for escape-hatch reasons and `GuardrailConfig` validation-call checks.

- Return one AST input per crawl root.
  - `g3rs-workspace-crawl` is already one explicit root snapshot.
  - Rejected multi-root fanout inside one crawl because that belongs to the old app router, not the package boundary.

## Files to modify

- `packages/rs/garde/g3rs-garde-ast-checks/crates/types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/test_support.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/select.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/*`
- `packages/rs/garde/g3rs-garde-ingestion/crates/types/src/error.rs`

## Summary

Built the real `g3rs-garde` AST lane. `g3rs-garde-ingestion` now implements `ingest_for_ast_checks`, and `g3rs-garde-ast-checks` now owns `RS-GARDE-10` for unreadable or malformed owned Rust source and `guardrail3.toml`.

## Decisions made

- Kept AST ingestion as path gathering only.
  - Why: reparsing in ingestion would make `RS-GARDE-10` unreachable through the real lane.
  - Rejected fail-fast parsing in ingestion for AST inputs.

- Required root `guardrail3.toml` at ingestion time, but left readability and parse failures to the AST package.
  - Why: the AST package already needs that file for escape-hatch reasons and `GuardrailConfig` validation-call checks.
  - Rejected silently dropping the file or treating present-but-bad policy input as absent.

- Filtered test and fixture Rust files during garde AST ingestion.
  - Why: the legacy garde source pass skipped test-owned files, and the extracted AST input type has no per-file `is_test` flag.

## Key files for context

- `.plans/2026-04-10-110435-build-garde-ast-lane.md`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/select.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/ast.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/rs_garde_10_input_failures/rule.rs`

## Verification

- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-ast-checks`
- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-ingestion`
- `git diff --check`

## Next steps

1. Run a full `test-attack` pass on the garde AST lane and harden any rule-coverage gaps it finds.
2. Decide whether `g3rs-garde-ast-checks` should eventually stop reading files directly and switch to source-content inputs.
3. Build the garde file-tree lane once the AST lane stops moving.

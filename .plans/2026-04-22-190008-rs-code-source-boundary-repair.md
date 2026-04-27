## Goal

Move `rs/code` source-file parsing out of `g3rs-code-source-checks` and into `g3rs-code-ingestion`, while preserving the current rule behavior and keeping raw source text available for comment-sensitive rules.

## Approach

1. Add an ingestion-owned parsed source state to the `g3rs-code-types` source input contract.
   - Keep the existing file metadata and raw content.
   - Add a parsed/invalid state so the source-check package no longer needs to call `syn::parse_file`.
2. Write a red run-sidecar test in `g3rs-code-source-checks`.
   - Build an input whose raw content is intentionally unparsable but whose prebound parsed AST is valid.
   - Prove `run.rs` should dispatch the prebound AST instead of reparsing raw content.
3. Update `g3rs-code-ingestion`.
   - Parse the file once during ingestion.
   - Route parse failures into the source input instead of leaving parsing to checks.
4. Update `g3rs-code-source-checks`.
   - Remove `support::parse_input`.
   - Make `run.rs` dispatch from the prebound parsed state or the prebound parse failure.
   - Keep the existing comment-text helpers and rule APIs intact.
5. Update affected tests and assertions.
   - Ingestion tests should assert the new parsed state.
   - Source-check helper constructors should build parsed inputs explicitly.

## Key decisions

- Keep raw `content` on the source input.
  - Reason: several live code rules still inspect exact source lines and same-line comments.
- Do not fan rules out into AST-derived mini inputs in this repair.
  - Reason: the confirmed defect is parse ownership, not per-rule AST projection.
- Store parse failures on the source input instead of making ingestion drop the file.
  - Reason: `g3rs-code/ast-30-input-failures` is a real rule and should still fire from ingestion-owned failure state.

## Alternatives considered

- Remove raw source content and force all rules onto AST-only facts.
  - Rejected: too broad for this repair; several current rules legitimately inspect source text.
- Leave parsing in `g3rs-code-source-checks` because it happens only once.
  - Rejected: still violates the orchestrator boundary. Parse-once work belongs in ingestion.

## Files to modify

- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-types/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/lib.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/run.rs`

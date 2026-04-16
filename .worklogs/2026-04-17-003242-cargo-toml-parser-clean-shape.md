Summary
- Removed the root schema-type aliases from `packages/parsers/cargo-toml-parser` so the clean public shape is `cargo_toml_parser::types::...`.
- Updated the current Rust callers to use the `types` module and verified the parser package plus the affected workspaces still compile and validate.

Decisions made
- Removed the root type aliases instead of preserving them with explicit `pub type` aliases.
  Why: the aliases kept the old mixed facade alive and weakened the clean split between parser entrypoints and schema types.
  Rejected: keeping compatibility aliases at the root just to avoid caller churn.
- Kept `parse`, `from_path`, and `Error` at the root.
  Why: parser entrypoints belong in the root facade; only the schema types needed to move under `types`.
  Rejected: moving the parser API itself under `types`, which would blur data types with runtime behavior.
- Fixed the repo callers directly instead of adding another facade layer.
  Why: the clean shape needs one obvious import path, not another transition shim.
  Rejected: introducing transitional re-exports or helper modules.

Key files for context
- `packages/parsers/cargo-toml-parser/src/lib.rs`
- `packages/parsers/cargo-toml-parser/src/types.rs`
- `.plans/2026-04-17-002433-cargo-toml-parser-clean-shape.md`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`

Next steps
- Continue package by package under `packages/parsers`.
- Keep the same stop condition: fix package-local debt, stop only when the next remaining issue is not clearly package debt or a valid check.

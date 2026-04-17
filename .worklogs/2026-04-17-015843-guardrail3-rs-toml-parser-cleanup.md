# Summary
Cleaned `packages/parsers/guardrail3-rs-toml-parser` to the current sibling-crate parser package shape and removed the old root-type API usage from touched callers. The package now passes its workspace tests and validates with no findings.

# Decisions made
- Flattened the old `crates/parser/{runtime,assertions,types}` layout to `crates/{runtime,assertions,types}` so the parser package matches the clean shape used across the parser family.
- Kept the root facade minimal: only `parse`, `from_path`, and `Error` at the root, with schema types under `guardrail3_rs_toml_parser::types`.
- Rewrote touched callers to use `guardrail3_rs_toml_parser::types::...` instead of preserving root-level type aliases.
- Moved runtime parser tests onto shared assertions and local helpers so sidecar tests no longer own result-shape proof or reach across their boundary directly.
- Added the missing library global-state bans in `clippy.toml` instead of introducing local lint escape hatches.

# Key files for context
- `packages/parsers/guardrail3-rs-toml-parser/Cargo.toml`
- `packages/parsers/guardrail3-rs-toml-parser/src/lib.rs`
- `packages/parsers/guardrail3-rs-toml-parser/src/types.rs`
- `packages/parsers/guardrail3-rs-toml-parser/guardrail3-rs.toml`
- `packages/parsers/guardrail3-rs-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/runtime/src/parser_tests/helpers.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/lib.rs`

# Next steps
- Continue sweeping `packages/parsers` and `packages/shared` package roots one by one.
- Keep removing old root parser type imports instead of preserving compatibility aliases.
- Stop only when the next issue is not clearly package debt or a clearly valid rule.

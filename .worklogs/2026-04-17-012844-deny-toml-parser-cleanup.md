# Summary
Cleaned `packages/parsers/deny-toml-parser` to the current sibling-crate package shape and removed stale root-type API usage across repo callers. The package now passes its workspace tests and validates with no findings.

# Decisions made
- Moved the parser package to `crates/runtime`, `crates/assertions`, and `crates/types` to match the clean parser package shape instead of keeping the old `crates/parser/...` nesting.
- Kept the root facade minimal: parse API at the root, types under `deny_toml_parser::types`, with no backward-compat root type aliases.
- Fixed downstream callers to use `deny_toml_parser::types::...` rather than reintroducing root-level type exports.
- Moved parser shape assertions out of `crates/runtime/src/parser_tests/parsing.rs` into `crates/assertions/src/parser.rs` so sidecar tests call shared proof instead of owning result-shape assertions locally.
- Routed parser test error cases through the sidecar helper so the sidecar stops reaching across its boundary directly to `parse`.
- Added the required global-state type bans to `clippy.toml` instead of introducing local lint escape hatches.

# Key files for context
- `packages/parsers/deny-toml-parser/Cargo.toml`
- `packages/parsers/deny-toml-parser/src/lib.rs`
- `packages/parsers/deny-toml-parser/src/types.rs`
- `packages/parsers/deny-toml-parser/guardrail3-rs.toml`
- `packages/parsers/deny-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/deny-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/deny-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/deny-toml-parser/crates/runtime/src/parser_tests/helpers.rs`
- `packages/parsers/deny-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/deny-toml-parser/crates/types/src/lib.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support/identities.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support/unknown_keys.rs`

# Next steps
- Continue parser cleanup package by package, starting with `packages/parsers/cliff-toml-parser`.
- Keep removing old root-type parser imports in callers instead of preserving compatibility aliases.
- Stop only when the next remaining issue is not clearly package debt or a clearly valid rule.

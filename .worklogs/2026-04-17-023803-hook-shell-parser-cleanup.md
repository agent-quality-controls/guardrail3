Summary

Normalized `packages/parsers/hook-shell-parser` to the current package shape and removed the last validation findings. The package now validates clean, and the downstream hooks packages were updated to use the cleaned `hook-shell-parser::types` field-based API instead of removed getter shims.

Decisions made

- Kept the clean root API shape: root exports behavior (`parse_script`, `command_query`) and the data model stays under `hook_shell_parser::types`.
- Kept the types crate passive. Instead of reintroducing compatibility getters, downstream hooks callers were rewritten to use public fields directly.
- Reshaped `command_query` into a real directory module with `command_query/mod.rs` as facade and `command_query/api.rs` as the logic owner. This satisfies the filetree rule without pushing logic back into a facade.
- Renamed the shared assertions ownership to match the logic owner: `crates/assertions/src/command_query/api.rs`.
- Replaced public-field expectation bags in assertions with explicit constructors so the assertion API stays narrow and rule-compliant.
- Fixed the broader fallout from the passive type cleanup in hooks packages instead of adding backward-compat aliases.

Key files for context

- `packages/parsers/hook-shell-parser/Cargo.toml`
- `packages/parsers/hook-shell-parser/src/lib.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/lib.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/parser.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/mod.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/assertions/src/parser.rs`
- `packages/parsers/hook-shell-parser/crates/assertions/src/command_query/mod.rs`
- `packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/...`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/selection.rs`

Next steps

- Commit this slice only.
- Continue parser cleanup package by package from `packages/parsers/hook-shell-parser` to the next failing parser root.
- Stop if the next remaining finding is a rule defect rather than package debt.

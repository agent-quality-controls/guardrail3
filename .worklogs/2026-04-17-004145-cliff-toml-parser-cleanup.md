Summary
- Normalized `packages/parsers/cliff-toml-parser` onto the current sibling-crate package shape with `crates/runtime`, `crates/assertions`, and `crates/types`.
- Moved the facade to the clean `types` module shape, fixed the parser sidecars to use shared proof, and updated the current repo callers to `cliff_toml_parser::types::...`.

Decisions made
- Flattened `crates/parser/{runtime,assertions,types}` into sibling crates under `crates/`.
  Why: the nested parser crate layout triggered the same test and arch failures already cleaned out of the other parser packages.
  Rejected: keeping the old layout and patching around each rule signal.
- Removed root type re-exports from the facade.
  Why: the clean shape is parser functions at the root and schema types under `types`.
  Rejected: preserving root aliases for compatibility.
- Kept the typed `Error` explicit in `runtime/src/parser.rs`.
  Why: the code rule needs the public error surface stated directly in the source, not hidden behind a re-export.
  Rejected: relying on `use crate::Error;` and leaving the rule to infer it.
- Kept the centralized `std::fs::read_to_string` and `toml::from_str` boundaries but switched to the inline `reason = ...` allow form already used by the clean parser packages.
  Why: that is the accepted package-local exception shape for parser/runtime boundary code.
  Rejected: leaving the old comment-based allow form, which still produced code-family warnings.

Key files for context
- `packages/parsers/cliff-toml-parser/Cargo.toml`
- `packages/parsers/cliff-toml-parser/guardrail3-rs.toml`
- `packages/parsers/cliff-toml-parser/src/lib.rs`
- `packages/parsers/cliff-toml-parser/src/types.rs`
- `packages/parsers/cliff-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/cliff-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/cliff-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/cliff-toml-parser/crates/assertions/src/parser.rs`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `.plans/2026-04-17-001209-cliff-toml-parser-cleanup.md`

Next steps
- Continue package by package under `packages/parsers`.
- The next parser to check is `packages/parsers/clippy-toml-parser`.
- Stop only when the next remaining issue is not clearly package-local debt or a valid check.

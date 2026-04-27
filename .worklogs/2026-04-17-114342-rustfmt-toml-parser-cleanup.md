Summary
- Normalized `packages/parsers/rustfmt-toml-parser` to the clean sibling-crate parser shape.
- Removed root type exports, moved downstream callers to `rustfmt_toml_parser::types::...`, and moved parser sidecar proof fully into the shared assertions crate.
- Validation now ends only on the two intentional centralized boundary warnings in `crates/runtime/src/fs.rs` and `crates/runtime/src/parser.rs`.

Decisions made
- Kept the parser package unpublished. This removes fake release debt for an internal parser package instead of adding README/docs.rs churn to satisfy publish-only checks.
- Removed the local `struct_excessive_bools` escape hatch from `RustfmtToml` and used the exact waiver path instead. The schema mirror stays direct, and the exception is visible in package policy rather than hidden in the type file.
- Did not leave backward-compat root type aliases. The root crate now exposes behavior only, and all typed access goes through `rustfmt_toml_parser::types`.

Key files for context
- `packages/parsers/rustfmt-toml-parser/Cargo.toml`
- `packages/parsers/rustfmt-toml-parser/guardrail3-rs.toml`
- `packages/parsers/rustfmt-toml-parser/src/lib.rs`
- `packages/parsers/rustfmt-toml-parser/src/types.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/types/src/rustfmt_toml.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/code/g3rs-code-types/src/types.rs`

Next steps
- Commit this slice by itself.
- Move to `packages/shared/guardrail3-check-types` and clean it package by package.
- Keep treating `g3rs-code/ast-04-item-level-allow-with-reason` centralized parser/fs warnings as visible escape-hatch inventory unless the user asks for code-family waiver support there too.

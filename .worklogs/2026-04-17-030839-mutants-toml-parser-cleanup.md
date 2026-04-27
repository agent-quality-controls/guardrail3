Summary

Normalized `packages/parsers/mutants-toml-parser` to the current sibling-crate package shape and removed all package-debt findings. The only remaining signals are the 2 intentional `g3rs-code/ast-04-item-level-allow-with-reason` inventory warnings on the centralized fs and parser boundary allows.

Decisions made

- Moved the package from `crates/parser/...` to `crates/{runtime,assertions,types}` to match the current parser package shape.
- Removed root type exports and put the data model under `mutants_toml_parser::types::...`.
- Kept the types crate passive and shared, with the large schema-mirror struct handled by exact waivers instead of local compatibility shims.
- Moved parser-side proof entirely into the shared assertions crate and deleted local parser test helpers that escaped the owned module boundary.
- Kept the centralized `toml::from_str` and centralized fs boundary allows visible as warnings instead of trying to hide them with compatibility code.

Key files for context

- `packages/parsers/mutants-toml-parser/Cargo.toml`
- `packages/parsers/mutants-toml-parser/src/lib.rs`
- `packages/parsers/mutants-toml-parser/src/types.rs`
- `packages/parsers/mutants-toml-parser/clippy.toml`
- `packages/parsers/mutants-toml-parser/deny.toml`
- `packages/parsers/mutants-toml-parser/guardrail3-rs.toml`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/mutants-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/mutants-toml-parser/crates/types/src/lib.rs`
- `packages/parsers/mutants-toml-parser/crates/types/src/mutants_toml.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-types/src/types.rs`

Next steps

- Commit this slice with the remaining 2 intentional `g3rs-code/ast-04-item-level-allow-with-reason` warnings.
- Continue to the next parser package root.
- Stop only if the next remaining issue is a real rule defect rather than package debt.

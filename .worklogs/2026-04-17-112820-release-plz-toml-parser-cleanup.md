Summary

Normalized `packages/parsers/release-plz-toml-parser` to the current sibling-crate parser package shape and removed the package-debt errors. The only remaining signals are the 2 intentional `g3rs-code/ast-04-item-level-allow-with-reason` inventory warnings on the centralized fs and parser boundary allows.

Decisions made

- Moved the package from `crates/parser/...` to `crates/{runtime,assertions,types}` to match the clean parser package shape.
- Removed the old root type dump. The root crate now exports behavior at the root and typed data under `release_plz_toml_parser::types`.
- Moved parser-side proof and tempfile-based parse fixtures into the shared assertions crate so the parser sidecar no longer imports sibling local modules.
- Kept the types crate passive and explicitly unpublished instead of preserving root type aliases or compatibility exports.
- Updated touched release-family callers to use `release_plz_toml_parser::types::ReleasePlzToml`.
- Kept the centralized `toml::from_str` and centralized fs boundary allows visible as warnings instead of hiding them.

Key files for context

- `packages/parsers/release-plz-toml-parser/Cargo.toml`
- `packages/parsers/release-plz-toml-parser/src/lib.rs`
- `packages/parsers/release-plz-toml-parser/src/types.rs`
- `packages/parsers/release-plz-toml-parser/guardrail3-rs.toml`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/release-plz-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/release-plz-toml-parser/crates/types/src/release_plz_toml.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-types/src/lib.rs`

Next steps

- Commit this slice only.
- Continue to the next parser package root.
- Stop only if the next remaining issue is a real rule defect rather than package debt.

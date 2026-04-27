Summary

Normalized `packages/parsers/rust-toolchain-toml-parser` to the current sibling-crate parser package shape and removed the package-debt errors. The only remaining signals are the 2 intentional `g3rs-code/ast-04-item-level-allow-with-reason` inventory warnings on the centralized fs and parser boundary allows.

Decisions made

- Moved the package from `crates/parser/...` to `crates/{runtime,assertions,types}` to match the clean parser package shape.
- Removed the old root type dump. The root crate now exports behavior at the root and typed data under `rust_toolchain_toml_parser::types`.
- Moved parser-side proof and tempfile-based parse fixtures into the shared assertions crate so the parser sidecar no longer imports sibling local modules.
- Kept the types crate passive and explicitly unpublished instead of preserving root type aliases or compatibility exports.
- Updated touched toolchain/fmt/code callers to use `rust_toolchain_toml_parser::types::RustToolchainToml`.
- Kept the centralized `toml::from_str` and centralized fs boundary allows visible as warnings instead of hiding them.

Key files for context

- `packages/parsers/rust-toolchain-toml-parser/Cargo.toml`
- `packages/parsers/rust-toolchain-toml-parser/src/lib.rs`
- `packages/parsers/rust-toolchain-toml-parser/src/types.rs`
- `packages/parsers/rust-toolchain-toml-parser/guardrail3-rs.toml`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/lib.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/types/src/rust_toolchain_toml.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/code/g3rs-code-types/src/types.rs`

Next steps

- Commit this slice only.
- Continue to the next parser package root.
- Stop only if the next remaining issue is a real rule defect rather than package debt.

# Summary

Moved code-family Rust source parsing out of `g3rs-code-ingestion` and out of the public `g3rs-code-types` contract. `G3RsCodeSourceChecksInput` is now serializable raw source metadata, while `g3rs-code-source-checks` parses the raw source once before dispatching pure source rules.

# Decisions Made

- Removed `G3RsCodeParsedSourceState` instead of adding adapters or `#[serde(skip)]`, because `syn::File` is not serializable and should not be in the fixture-facing public type.
- Removed the `syn` dependency from `g3rs-code-ingestion-runtime`, because ingestion now reads and classifies source files but does not parse Rust syntax.
- Kept `syn::parse_file` behind the source-checks parse support module, then dispatches existing rules through `CodeSourceRuleInput`.
- Regenerated fixture test ledgers after test names and line numbers changed; fixture3 replay output still matches approved output.

# Key Files For Context

- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run.rs`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `.plans/2026-05-15-165700-code-source-parse-boundary.md`
- `.plans/2026-05-15-165700-code-source-parse-boundary.md.manifest.toml`

# Verification

- `cargo check --workspace --all-targets --all-features` passed in `packages/rs/code/g3rs-code-types`.
- `cargo check --workspace --all-targets --all-features` passed in `packages/rs/code/g3rs-code-ingestion`.
- `cargo check --workspace --all-targets --all-features` passed in `packages/rs/code/g3rs-code-source-checks`.
- `cargo test --workspace --all-targets --all-features` passed in `packages/rs/code/g3rs-code-types`.
- `cargo test --workspace --all-targets --all-features` passed in `packages/rs/code/g3rs-code-ingestion`.
- `cargo test --workspace --all-targets --all-features` passed in `packages/rs/code/g3rs-code-source-checks`.
- `g3rs validate --path packages/rs/code/g3rs-code-types` passed with existing release warnings.
- `g3rs validate --path packages/rs/code/g3rs-code-ingestion` passed with existing code/release warnings.
- `g3rs validate --path packages/rs/code/g3rs-code-source-checks` passed with existing release warnings.
- `g3rs validate-repo` passed with `No findings`.
- `scripts/behavior/verify-all.sh` passed; fixture3 matched 36 validate fixtures and 9 validate-repo fixtures.

# Next Steps

- Continue making remaining fixture-facing family input structs derive `Serialize`.
- Treat non-serializable public fields as architecture defects unless there is a documented external boundary that cannot be serialized directly.

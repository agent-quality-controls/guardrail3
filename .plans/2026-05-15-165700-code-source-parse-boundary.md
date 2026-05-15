# Goal

Make code-family source-check inputs serializable without duplicating parsing inside individual rules.

# Approach

- Remove `syn::File` from `g3rs-code-types`.
- Keep `G3RsCodeSourceChecksInput` as the serializable public ingestion output.
- Move the `syn::parse_file` call into `g3rs-code-source-checks` support code.
- Parse once per source-check input before dispatching rules.
- Keep individual rule files unchanged unless their tests construct `G3RsCodeParsedSourceState` directly.
- Do not write adapters, exporters, manual serializers, fixture-only structs, or `#[serde(skip)]`.

# Files To Modify

- `packages/rs/code/g3rs-code-types/Cargo.toml`
- `packages/rs/code/g3rs-code-types/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run_source.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/input_failures/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/core.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run_tests/cases.rs`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- affected `Cargo.lock` files updated by Cargo after dependency changes

# Verification

- `cargo check --manifest-path packages/rs/code/g3rs-code-types/Cargo.toml`
- `cargo test --manifest-path packages/rs/code/g3rs-code-types/Cargo.toml`
- `cargo check --manifest-path packages/rs/code/g3rs-code-ingestion/Cargo.toml`
- `cargo test --manifest-path packages/rs/code/g3rs-code-ingestion/Cargo.toml`
- `cargo check --manifest-path packages/rs/code/g3rs-code-source-checks/Cargo.toml`
- `cargo test --manifest-path packages/rs/code/g3rs-code-source-checks/Cargo.toml`
- `g3rs validate --path packages/rs/code/g3rs-code-types`
- `g3rs validate --path packages/rs/code/g3rs-code-ingestion`
- `g3rs validate --path packages/rs/code/g3rs-code-source-checks`
- `scripts/behavior/verify-all.sh`

# G3RS Test Activation Lenient Source Parse

## Goal

Fix the test-family ingestion layer so malformed Rust source files do not abort the whole test family before `g3rs-test/source-input-failures` and `g3rs-test/filetree-input-failures` can report through normal rule output.

## Bug

`g3rs_test_ingestion::activation::summarize_root` parses Rust files to decide whether a package has tests.

Current behavior:

- malformed Rust in `tests/*.rs` returns `IngestionError::ParseFailed`
- `family-runner-test` stops before filetree/source checks run
- the public CLI prints `test: ParseFailed ...` on stderr
- the semantic input-failure rules cannot emit

This is the wrong layer. Activation is a routing summary. Source/filetree parse failures already have dedicated rule IDs.

## Fix

Modify `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/activation.rs`.

Behavior:

- unreadable files may still return `Unreadable`, because no later source analysis can read them reliably
- malformed readable Rust files must not return `ParseFailed` from activation
- if the malformed file path itself proves a test surface, set `summary.has_tests = true`
- if the malformed file path does not prove a test surface, skip it and let source/filetree analysis report the parse failure

Modify `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs`.

Behavior:

- module-directory discovery must skip malformed readable Rust source files
- it must still parse valid Rust files and discover module directories from them
- arch has no source input-failure rule, so it must not hide `g3rs-code`, `g3rs-garde`, or `g3rs-test` input-failure findings

Path-proven test surfaces:

- `tests/**/*.rs`
- internal sidecar paths under `src/**/_tests/**`
- assertions source paths
- test support source paths

## Test

Add a unit test in `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run_tests/pipeline.rs`.

The test must:

- create a minimal Rust package
- add `tests/broken.rs` with invalid Rust
- call config, filetree, and source ingestion/check pipelines
- prove config ingestion does not return `ParseFailed`
- prove `g3rs-test/source-input-failures` emits for `tests/broken.rs`
- prove `g3rs-test/filetree-input-failures` emits for `tests/broken.rs`

Add a unit test in `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`.

The test must:

- create a workspace with one package
- add one valid `src/lib.rs`
- add one malformed readable `src/broken.rs`
- call arch filetree ingestion through the existing helper
- prove ingestion succeeds

## Verification

Run:

```sh
cargo test --manifest-path packages/rs/test/g3rs-test-ingestion/crates/runtime/Cargo.toml
cargo test --manifest-path packages/rs/arch/g3rs-arch-ingestion/crates/runtime/Cargo.toml
```

Then continue the L45 behavior fixture implementation.

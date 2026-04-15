Goal
- Clean `packages/rs/code/g3rs-code-ingestion` to `No findings.` without changing rules again.

Approach
- Add the missing workspace-root policy files and `guardrail3-rs.toml`.
- Mark the workspace and child crates explicitly unpublished where release burden should stand down.
- Slim `crates/types` to the real shared boundary only: the ingestion error. Move shared code-family types usage back to `g3rs-code-types`.
- Rename `ingest_tests` to `run_tests`, because the tested entry points live in `run.rs`.
- Add `crates/assertions/src/run.rs` and move final proof there, so sidecars stop doing local result checks and stop importing sibling assertion modules.
- Rewire sidecars to call `crate::run::ingest_for_*` and the shared assertions crate instead of local wrappers or direct `CheckResult` field access.

Key decisions
- Keep the `types` crate only if it still owns a real boundary after cleanup. Here that real boundary is the ingestion error.
- Follow the cleaned `cargo-ingestion` pattern instead of inventing a new shape.
- Treat the remaining failures as package debt unless another direct rule contradiction appears during the cleanup.

Files to modify
- `packages/rs/code/g3rs-code-ingestion/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-ingestion/clippy.toml`
- `packages/rs/code/g3rs-code-ingestion/deny.toml`
- `packages/rs/code/g3rs-code-ingestion/rustfmt.toml`
- `packages/rs/code/g3rs-code-ingestion/rust-toolchain.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/*`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/error.rs`

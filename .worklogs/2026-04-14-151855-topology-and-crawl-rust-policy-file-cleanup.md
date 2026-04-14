Summary

- Removed dead `guardrail3.toml` handling from active topology and workspace-crawl code.
- Reattached `guardrail3-rs.toml` to `garde` in topology so the live Rust policy file remains fully classified.

Decisions made

- Removed `GuardrailToml` from topology public types instead of keeping a dead variant.
  - Reason: the Rust-only model has no live universal guardrail file.
- Classified `guardrail3-rs.toml` for `garde` in topology alongside `cargo` and `deps`.
  - Reason: garde activation and waivers now come from the Rust policy file.
- Removed `guardrail3.toml` from workspace-crawl recovery.
  - Rejected: keeping it as a "harmless" recoverable legacy file, because that would preserve dead semantics in active infrastructure.

Key files for context

- `.plans/2026-04-14-151704-topology-and-crawl-rust-policy-file-cleanup.md`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/recovery.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/ignore_state.rs`

Next steps

- Move to the remaining stale Rust-only cleanup in `hooks`, `release`, and `fuzz`.
- Keep negative regressions that prove `guardrail3.toml` is ignored, but do not allow any live code path to classify or recover it.

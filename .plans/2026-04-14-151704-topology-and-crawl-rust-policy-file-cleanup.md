Goal

- Remove dead `guardrail3.toml` handling from active topology and workspace-crawl code.
- Keep topology family-file classification accurate for the Rust-only model by attaching `guardrail3-rs.toml` to `garde`.

Approach

- Add or update tests first in topology ingestion and workspace-crawl recovery to prove:
  - `guardrail3.toml` is no longer recovered from ignored space.
  - topology no longer classifies `guardrail3.toml` as a family file.
  - topology classifies `guardrail3-rs.toml` for `garde` as well as the existing Rust families.
- Remove `GuardrailToml` from topology public types and ingestion classification.
- Replace `guardrail3.toml` with `guardrail3-rs.toml` in workspace-crawl recovery rules.
- Re-run package tests and app tests, then adversarially review the slice.

Key decisions

- Fix topology and workspace-crawl together.
  - Reason: both are part of the same dead-config surface and should converge in one step.
- Keep the change narrow.
  - No new family semantics, only removal of dead universal-config handling and correct reattachment of the live Rust policy file.

Files to modify

- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/recovery.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/ignore_state.rs`

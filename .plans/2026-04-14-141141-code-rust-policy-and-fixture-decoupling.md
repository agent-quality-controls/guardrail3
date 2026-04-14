Goal

- Remove the remaining `code` family dependency on deleted old-app test fixtures.
- Remove the dead `guardrail3.toml` filename from the active `code` package surface.
- Keep the `code` package Rust-only and package-local.

Approach

- Add or tighten tests first:
  - config ingestion should select `guardrail3-rs.toml`, not `guardrail3.toml`
  - exception comment inventory should cover `guardrail3-rs.toml`
  - source pipeline tests should use package-owned inline fixtures instead of deleted old-app `include_str!` paths
- Update `g3rs-code-ingestion`:
  - replace `guardrail3.toml` with `guardrail3-rs.toml` in supported config filenames
  - parse `guardrail3-rs.toml` into the existing typed Rust policy kind
- Decide the public type naming:
  - if the existing `Guardrail3Toml` enum variant is now misleading, rename it to `Guardrail3RsToml`
  - update all direct tests accordingly
- Remove old-app fixture includes from `ingest_tests/pipeline.rs`
  - inline or package-local constants only

Key decisions

- The file name must be `guardrail3-rs.toml`.
  - The old universal `guardrail3.toml` is dead.
- Package tests must not depend on deleted app fixtures.
  - Active packages should remain self-contained even under repository quarantine.

Files to modify

- `.plans/2026-04-14-141141-code-rust-policy-and-fixture-decoupling.md`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/{basic.rs,pipeline.rs}`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- any direct `code` tests referencing the old enum variant or old file name

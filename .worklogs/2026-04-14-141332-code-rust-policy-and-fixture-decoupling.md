Summary

- Removed the remaining `code` family dependency on deleted old-app fixtures.
- Switched `code` config ingestion from dead `guardrail3.toml` to Rust-only `guardrail3-rs.toml`.

Decisions made

- Renamed the public config-file variant from `Guardrail3Toml` to `Guardrail3RsToml`.
  - The old universal file name is dead and should not stay in active package contracts.
- Replaced old `include_str!(apps/guardrail3/...)` fixtures with package-local inline fixtures.
  - Active package tests must remain self-contained after old-app quarantine.
- Kept the generated `packages/rs/code/g3rs-code-source-checks/Cargo.lock` update with the slice.
  - It reflects the actual workspace dependency graph after the package-local changes.

Key files for context

- `.plans/2026-04-14-141141-code-rust-policy-and-fixture-decoupling.md`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

Next steps

- Move to the next family still carrying dead `guardrail3.toml` debt.
- Keep rejecting any active-package test that reaches into deleted old-app fixtures.

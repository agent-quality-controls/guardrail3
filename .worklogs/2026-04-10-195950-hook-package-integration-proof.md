# Summary

Added package-level integration proofs for the new `hooks-rs` source lane. The new tests prove three things with the extracted packages only: broken hooks emit hook findings, a hook that runs Rust validation lines up with real `code` source failures, and a hook that covers Rust config changes lines up with real `clippy` config failures.

# Decisions made

- Put the proof in `g3rs-hooks-rs-ingestion` runtime tests instead of the legacy app runtime.
  - Why: the package architecture is the source of truth now.
  - Rejected: app-level runtime tests, because they still route through legacy families.
- Use real downstream package pipelines inside the hook integration tests.
  - Why: the proof needed to show hook coverage and downstream family failures on the same repo snapshot.
  - Rejected: only asserting hook rule IDs, because that does not prove the downstream breakage is actually surfaced by the extracted packages.

# Key files

- `.plans/2026-04-10-195250-hook-integration-proof.md`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/crates/runtime/src/ingest_tests/integration.rs`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/crates/runtime/Cargo.toml`

# Next steps

- Add the same package-level proof for `hooks-shared` if we want structural hook misconfiguration covered in the same style.
- When the legacy app runtime is retired, add one top-level package-only orchestration proof that composes hook and downstream package lanes together without the app.

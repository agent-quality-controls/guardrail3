Goal

Clean `packages/rs/garde/g3rs-garde-ingestion` to the current package shape so package tests pass and `guardrail3-rs validate --path packages/rs/garde/g3rs-garde-ingestion` returns no findings without changing rules.

Approach

- Normalize the package root to the same baseline as other cleaned ingestion packages:
  - make publish intent explicit
  - add root policy files
  - remove stale local types dependency shape if it is no longer needed
- Normalize crate manifests under `crates/`:
  - explicit `publish`
  - docs.rs metadata
  - correct runtime/assertions dependency edges
- Clean the runtime test layout:
  - replace `#[cfg(test)] mod ingest_tests;` with owned sidecar on `lib.rs`
  - keep `ingest_tests/mod.rs` facade-only
  - move helper functions out of `source/mod.rs`
  - stop sidecars from escaping the owned module boundary
- Add shared assertions coverage for ingest tests:
  - create `crates/assertions/src/ingest.rs`
  - move semantic result assertions there
  - update sidecars to call the shared proof surface
- Remove stale test imports of local types crates and point everything at the correct shared/runtime crates.
- Re-run package tests and package validate. Only stop if a real rule contradiction appears.

Key decisions

- Use a cleaned ingestion package such as `g3rs-clippy-ingestion` as the package-shape reference, but keep garde-specific runtime behavior local.
- Do not change rules unless the package reaches a genuine contradiction after the stale layout and assertion debt is removed.

Files to modify

- `packages/rs/garde/g3rs-garde-ingestion/Cargo.toml`
- `packages/rs/garde/g3rs-garde-ingestion/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/**`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/**`
- `packages/rs/garde/g3rs-garde-ingestion/crates/types/**`
- root policy files under `packages/rs/garde/g3rs-garde-ingestion/`

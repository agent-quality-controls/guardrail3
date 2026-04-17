Goal
- Make `packages/rs/topology/g3rs-topology-ingestion` validate clean under all active families without changing any rules.

Approach
- Normalize the root package shell and member manifests to the current internal ingestion package shape with explicit non-publish intent, root policy files, and a package-level `guardrail3-rs.toml`.
- Replace the old `ingest_tests` sidecar with an owned `run` module and `run_tests/` sidecars, add a small assertions crate surface for shared end-to-end proofs, and tighten weak test expect messages.
- Isolate direct filesystem reads behind a tiny `fs` module and gate the public types facade, matching the cleaned ingestion siblings.

Key Decisions
- Keep the runtime layout centered on `run.rs` and `view.rs`; do not introduce extra crates or refactor ingestion logic beyond what is needed to restore clean package boundaries.
- Reuse the single `run` assertions module pattern from `g3rs-hooks-ingestion` instead of creating many tiny assertion modules, because this package has one ingestion entrypoint and end-to-end proof surface.

Files To Modify
- `packages/rs/topology/g3rs-topology-ingestion/Cargo.toml`
- `packages/rs/topology/g3rs-topology-ingestion/guardrail3-rs.toml`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/**`
- `packages/rs/topology/g3rs-topology-ingestion/crates/assertions/**`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/**`

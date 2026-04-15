Goal
- Split the repo-root release workflow checks out of the workspace-local release package.
- Keep the three rules as live code in a new package instead of archiving them.
- Add a named stub API for future repo-root ingestion: `ingest_for_repo_root_checks`.

Approach
- Remove workflow rules `12/13/14` from active `g3rs-release-config-checks` runtime wiring.
- Create a new live package `packages/rs/release/g3rs-release-repo-root-checks` with the usual facade/runtime/types/assertions shape.
- Rename the moved rules to `01/02/03` inside the new package and move their tests there as direct rule tests.
- Update the old release ingestion pipeline test so it stops expecting the moved rules.
- Add `ingest_for_repo_root_checks` to release ingestion and make it return a clear not-implemented error for now.
- Add a direct test that proves the stub returns that error.

Key decisions
- Keep the moved rules in `packages/rs`, not `legacy`, because they are still live future work.
- Do not wire the new package into `guardrail3-rs` yet. It has no real repo-root ingestion yet.
- Use a new ingestion error variant instead of overloading an existing normalization error.

Files to modify
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/types/src/error.rs`
- new package under `packages/rs/release/g3rs-release-repo-root-checks/**`

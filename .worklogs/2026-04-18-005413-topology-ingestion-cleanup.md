Summary

Cleaned `packages/rs/topology/g3rs-topology-ingestion` to the current package shape and removed the remaining topology-ingestion validation debt. The package now validates cleanly and its workspace tests pass.

Decisions made

- Moved the old `ingest_tests` sidecars under owned `run_tests` and wired them from `run.rs` so the runtime module owns its own sidecar tree.
- Added package root policy files and normalized publish/features/docs metadata across root, runtime, types, and assertions crates to match the cleaned sibling packages.
- Kept topology file-tree input proofs in the shared assertions crate, but collapsed them into the existing `run` assertions surface instead of introducing a sibling assertions module. This avoided a forbidden local edge and kept internal sidecars on the owned assertions path.
- Re-exported the topology file-tree input and related enums from the runtime crate so the assertions crate can prove runtime-owned inputs without depending directly on the non-member workspace package.
- Replaced the unreachable runtime `panic!` in owner attachment normalization with a debug assertion plus deterministic fallback to the validation root, removing the last package-local warning.
- Rewrote the pipeline and file-tree sidecars to use shared assertions, stronger `expect(...)` messages, and specific error-payload matches.

Key files for context

- `packages/rs/topology/g3rs-topology-ingestion/Cargo.toml`
- `packages/rs/topology/g3rs-topology-ingestion/guardrail3-rs.toml`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/view.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/fs.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run_tests/file_tree.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/assertions/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/lib.rs`

Next steps

- Commit this package cleanup as a standalone topology-ingestion change.
- Rerun the full package-root validate sweep, including warnings, to confirm whether only the parser warning-only packages remain.
- If parser warning-only packages still remain, clean those last `g3rs-code/ast-04-item-level-allow-with-reason` escape-hatch warnings so every package root is fully clean under all families.

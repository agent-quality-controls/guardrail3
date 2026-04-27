Summary

Cleaned `packages/rs/arch/g3rs-arch-ingestion` until both the package workspace tests and `validate` returned clean. The package now follows the current ingestion pattern: runtime-owned error, focused runtime modules, owned `x_tests` sidecars, and shared assertions files.

Decisions made

- Deleted `crates/types` because it was only a local wrapper around shared arch types plus the ingestion error.
  - Why: the clean pattern for ingestion packages is runtime-owned error plus direct use of the shared family types crate.
  - Rejected: keeping the wrapper and waiving the fake topology and arch edges.
- Split the old monolithic runtime into `config.rs`, `file_tree.rs`, `source.rs`, `workspace.rs`, `view.rs`, and `fs.rs`.
  - Why: this removes the oversized `run.rs`, keeps filesystem access behind one shim, and gives each ingestion lane an owned test sidecar.
  - Rejected: leaving the old large `run.rs` and patching around the structure rules.
- Replaced the stale `run_tests` sidecar with real owned sidecars:
  - `source_tests`
  - `config_tests`
  - `file_tree_tests`
  - Why: each sidecar now belongs to a real production file and final proof lives in the shared assertions crate.
  - Rejected: keeping the orphaned `run_tests` tree, because it kept triggering real `code` and `test` findings.
- Fixed the stale config pipeline expectation instead of changing the rule.
  - Why: under current arch rules, direct child and root-level sibling path edges are `g3rs-arch/no-boundary-crossing` inventory, while the missing shared flag is the real `g3rs-arch/shared-flag-required` error.
  - Rejected: changing arch rule behavior to satisfy one stale package test.

Key files for context

- `packages/rs/arch/g3rs-arch-ingestion/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-ingestion/src/lib.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/config.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/view.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/assertions/src/config.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/assertions/src/source.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/assertions/src/file_tree.rs`

Next steps

- Continue the package-by-package cleanup pass after `packages/rs/arch/g3rs-arch-ingestion`.
- Stop only on the next real rule contradiction or false positive.
- Reuse this package as the reference shape for arch ingestion workspaces with runtime-owned error, focused ingestion modules, and shared assertions by ingestion lane.

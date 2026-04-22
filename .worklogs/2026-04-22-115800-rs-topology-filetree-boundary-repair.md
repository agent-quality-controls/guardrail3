Summary
- Repaired the `rs/topology` file-tree seam by moving topology fact derivation out of `g3rs-topology-file-tree-checks` and into ingestion.
- Added a proving run test that fails if file-tree checks ignore precomputed inputs, then rewired the package so checks dispatch only over ingestion-owned facts.

Decisions made
- Kept the existing raw topology bag fields in `G3RsTopologyFileTreeChecksInput` for now and added precomputed atomic fact vectors beside them.
  - Why: the concrete package defect was check-local normalization in `g3rs-topology-file-tree-checks`. Fixing that package cleanly did not require widening this turn into a full topology type-surface rewrite.
  - Rejected: removing the raw fields in the same commit. That would have forced a larger family-wide refactor than the package repair actually needed.
- Moved the old `collect_facts` logic into ingestion as `file_tree_facts.rs`.
  - Why: this preserves the existing rule semantics while relocating discovery and normalization to the architecturally correct layer.
  - Rejected: partially migrating only one rule lane. That would have left mixed ownership inside the check package.
- Added a shared run assertions file for the new file-tree `run_tests` sidecar.
  - Why: `g3rs validate` correctly rejected direct `CheckResult` field inspection in the sidecar test. The assertions crate now owns the result-shape proof.

Key files for context
- `.plans/2026-04-22-114116-rs-topology-filetree-boundary-repair.md`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/file_tree_facts.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run_tests/cases.rs`

Next steps
- Revisit `g3rs-topology-types` and decide whether the remaining raw bag fields should be removed now that file-tree checks no longer consume them.
- Continue the Rust package-boundary repair on the next bag-heavy family package after topology.

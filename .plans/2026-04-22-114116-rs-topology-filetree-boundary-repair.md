Goal
- Move `rs/topology` file-tree normalization out of `g3rs-topology-file-tree-checks` and into ingestion so the check package only dispatches pure rules over precomputed topology facts.

Approach
- Read the live topology family boundary:
  - `packages/rs/topology/g3rs-topology-types/src/types.rs`
  - `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
  - `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs`
  - `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- Add failing tests first at the appropriate seam:
  - ingestion tests that prove nested-workspace, member-issue, escaping-member-path, and illegal-family-file facts are computed before checks run
  - file-tree check tests that prove `run.rs` only dispatches over precomputed inputs
- Narrow the family contract in `g3rs-topology-types`:
  - replace the big `G3RsTopologyFileTreeChecksInput` bag with explicit per-lane fact vectors
  - keep only rule-local atomic inputs in the checks package
- Move the current `TopologyFacts` derivation from file-tree `support.rs` into ingestion-owned code
  - if needed, add an ingestion-local helper module for the normalization logic
- Remove check-local normalization helpers that are no longer needed
- Run targeted tests and `g3rs validate` on touched packages
- Commit the repair with a worklog

Key decisions
- Keep rule-local input structs where they already exist instead of inventing a second parallel topology fact model.
  - Alternative rejected: create new duplicate ingestion-only structs and map them again in checks. That would keep needless translation layers.
- Keep only display-format helpers in file-tree check support.
  - Alternative rejected: leave `TopologyFacts` in checks and just rename it. That preserves the broken boundary.
- Fix the whole file-tree seam in one pass.
  - Alternative rejected: move only one lane, such as nested workspaces, and leave the rest bag-shaped. That would keep mixed ownership in the same package.

Files to modify
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/*` as needed for extracted normalization helpers and tests
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/*/rule_tests/*` if expected inputs or fixtures change

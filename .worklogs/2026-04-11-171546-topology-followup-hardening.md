Summary

Closed the follow-up topology migration gaps found by the latest adversarial review. The workspace-local topology package now handles descendant manifest failures visibly, normalizes `./` member paths correctly, rejects absolute member paths as escapes, and has stronger end-to-end coverage for nested workspaces and illegal placement.

Decisions made

- Added `RS-TOPOLOGY-07` to the extracted workspace-local topology slice. The previous boundary was too narrow: descendant manifest failures inside a valid pointed workspace are not ingestion-fatal, but they still weaken topology legality if ignored.
- Normalized leading `./` away before matching workspace members. This preserves valid Cargo syntax without widening semantics.
- Treated absolute member paths as escaping-member violations under `RS-TOPOLOGY-13`.
- Kept rule 16 root-sidecar allowances narrow:
  - root `.cargo/config*`
  - root `.cargo/deny.toml`
  - root `.cargo/mutants.toml`
  - root `.config/nextest.toml`
  Everything else still follows the existing placement rules.

Key files for context

- `.plans/2026-04-11-165737-topology-followup-hardening.md`
- `.plans/todo/checks/rs/topology.md`
- `.plans/by_family/rs/topology.md`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_07_required_inputs_fail_closed.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

Next steps

- Remove stale old-app ownership/docs drift that still says only `11/12/13/16` are the extracted workspace-local subset.
- Keep using topology for workspace-local legality only. Do not pull repo-global root placement policy into the package layer.

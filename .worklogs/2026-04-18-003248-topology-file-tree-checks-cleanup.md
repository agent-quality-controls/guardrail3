Summary
- Cleaned `g3rs-topology-file-tree-checks` into the current internal package shape and made the package validate clean under all active families.
- The main architectural fix was removing the shared generic topology issue input and giving each rule its own owned input type so sidecar tests can call only the owned rule module and the shared assertions crate.

Decisions Made
- Kept the package as one root facade with `runtime`, `types`, and `assertions` crates, matching the cleaned file-tree packages in other families.
- Removed the old generic `test_support` and replaced it with owned `rule_tests` directories so each rule sidecar only touches its owned rule module.
- Moved file-tree fact fan-out in `support.rs` onto per-rule inputs instead of the old shared `TopologyIssue` enum. This reduced package complexity and fixed the sidecar boundary problem at the source instead of papering over it in tests.

Key Files For Context
- `packages/rs/topology/g3rs-topology-file-tree-checks/Cargo.toml`
- `packages/rs/topology/g3rs-topology-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/assertions/src/common.rs`

Next Steps
- Clean `packages/rs/topology/g3rs-topology-ingestion`.
- Run a fresh full-repo validate sweep.
- Confirm that only the previously accepted parser warning-only packages remain non-clean.

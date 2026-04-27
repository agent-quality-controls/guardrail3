Summary

Added the missing topology attack coverage for UNC and backslash-absolute workspace member paths. This closes the remaining `g3rs-topology/member-paths-must-not-escape-root` branch gap without changing runtime logic.

Decisions made

- Kept the fix test-only.
  - Reason: the escape predicate already handled these forms; the gap was coverage, not behavior.
- Added both UNC and rooted-backslash path cases.
  - Reason: they exercise the same `starts_with('\\')` branch with distinct real-world path forms.

Key files for context

- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_13_member_paths_must_not_escape_root_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`

Next steps

- Audit `hexarch` with the same standard and decide whether its fake config/filetree lanes should be removed or built.

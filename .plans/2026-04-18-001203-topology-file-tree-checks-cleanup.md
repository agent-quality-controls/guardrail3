Goal
- Make `packages/rs/topology/g3rs-topology-file-tree-checks` validate clean under all active families without changing any rules.

Approach
- Normalize the root package shell to the current internal workspace shape with explicit non-publish intent, root policy files, and a package-level `guardrail3-rs.toml`.
- Remove the runtime dependency on the local `types` crate and depend directly on `g3rs-topology-types`, leaving the local `types` crate as a feature-gated facade for the root API.
- Convert each rule into an owned directory with `mod.rs`, `rule.rs`, and `rule_tests/`, delete the generic `test_support`, and move result proofs into one shared assertions module per rule.

Key Decisions
- Keep the runtime crate aggregated and waive `RS-ARCH-FILETREE-07` narrowly, matching the other cleaned file-tree packages.
- Use the standard shared assertions macro surface rather than keeping the ad hoc public `ExpectedRuleResult` struct with public fields.

Files To Modify
- `packages/rs/topology/g3rs-topology-file-tree-checks/Cargo.toml`
- `packages/rs/topology/g3rs-topology-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/topology/g3rs-topology-file-tree-checks/src/lib.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/**`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/assertions/**`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/types/**`

Summary

Cleaned `packages/rs/hooks/g3rs-hooks-file-tree-checks` to the current package shape. The package now uses root policy files, direct `g3rs-hooks-types` runtime dependencies, owned rule sidecars, and the shared `Finding`-based assertions surface. `cargo test` passes and `guardrail3-rs validate` reports `No findings.`

Decisions made

- Reused the cleaned `g3rs-hooks-config-checks` package shape instead of inventing a second hooks package pattern.
- Kept the local `crates/types` crate only as the package facade and stopped routing runtime code through it.
- Removed shared runtime `test_support` and moved test data builders into per-rule `rule_tests/helpers.rs` files so each rule owns its own test surface.
- Replaced the public `ExpectedRuleResult` bag with exact `Finding` assertions because file-tree rules need typed severity and file-presence checks across `Error`, `Warn`, and `Info`.

Key files for context

- `packages/rs/hooks/g3rs-hooks-file-tree-checks/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/assertions/src/common.rs`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/types/src/lib.rs`

Next steps

- Continue package-by-package on the remaining dirty hooks packages.
- Start with `packages/rs/hooks/g3rs-hooks-ingestion`, which is still on the older package shape and likely needs the same root policy and owned-sidecar normalization.

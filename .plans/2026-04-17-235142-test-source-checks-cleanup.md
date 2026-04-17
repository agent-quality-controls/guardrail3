Goal

Bring `packages/rs/test/g3rs-test-source-checks` to `No findings.` under the current guardrail families.

Approach

- Normalize the package shell to match the already-clean `g3rs-test-file-tree-checks` package:
  - root policy files
  - `guardrail3-rs.toml`
  - root `publish = false`
  - feature-gated facade exports
  - explicit member publish/docs/shared metadata
  - runtime depends on `g3rs-test-types` directly, not the local types crate
- Replace facade-owned and ad hoc `tests/` sidecars with owned `rule_tests/` directories for every rule module.
- Build the sibling assertions crate out so rule tests call shared proof helpers instead of asserting results locally.
- Split `parse/mod.rs` into facade + submodules and collapse the large flat parser records the same way as `g3rs-test-file-tree-checks`.
- Re-run workspace tests and package validate until the package is fully clean.

Key decisions

- Reuse the package shape from `g3rs-test-file-tree-checks` instead of inventing a second pattern for test-family packages.
- Preserve the current source-rule behavior while moving proof assertions into the assertions crate.
- Fix large parser structs by grouping repeated facts into nested records instead of adding waivers.

Files to modify

- `packages/rs/test/g3rs-test-source-checks/Cargo.toml`
- `packages/rs/test/g3rs-test-source-checks/src/lib.rs`
- `packages/rs/test/g3rs-test-source-checks/guardrail3-rs.toml`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/**`
- `packages/rs/test/g3rs-test-source-checks/crates/types/**`
- `packages/rs/test/g3rs-test-source-checks/crates/assertions/**`

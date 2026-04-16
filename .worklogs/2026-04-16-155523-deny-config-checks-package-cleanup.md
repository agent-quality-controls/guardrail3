Summary

Cleaned `packages/rs/deny/g3rs-deny-config-checks` to the current package rules. The package now passes both workspace tests and full `guardrail3-rs validate` with no findings.

Decisions made

- Removed the fake local `types` crate and wired the package directly to `g3rs-deny-types`.
- Kept the approved `x_tests` and `rule_tests` sidecar shape with `#[path = "..._tests/mod.rs"]` bridging for file modules.
- Moved grouped shared assertions into nested `crates/assertions/src/{group}/{rule}/rule.rs` paths so grouped sidecars use the owned shared proof path.
- Added a real `crates/test_support` crate for generic test input builders, then moved the hardcoded deny fixture strings back into owned sidecar `helpers.rs` files because shared `test_support` must stay generic.
- Split the old `support.rs` grab-bag into focused modules under `crates/runtime/src/support/`:
  - `findings.rs`
  - `expectations.rs`
  - `identities.rs`
  - `policy.rs`
  - `unknown_keys.rs`
- Replaced the large string `match` in `unknown_keys.rs` with table-driven lookup so the file no longer trips the string-dispatch rule.

Key files for context

- `packages/rs/deny/g3rs-deny-config-checks/Cargo.toml`
- `packages/rs/deny/g3rs-deny-config-checks/guardrail3-rs.toml`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support/mod.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/test_support/src/lib.rs`

Next steps

- Commit this package slice as a standalone cleanup.
- Move to the next package and keep cleaning package debt until the next real rule contradiction appears.

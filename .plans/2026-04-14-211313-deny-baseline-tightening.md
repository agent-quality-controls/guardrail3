# Goal
Make the active `deny` family baseline stricter and more correct by removing the stale default slack for `advisories.yanked` and `regex` ban wrappers. After the change, package workspaces that remove those carveouts should pass the deny family instead of being forced back to the old defaults.

# Approach
1. Add or update direct rule tests for `g3rs-deny/advisories-baseline` so the golden baseline requires `yanked = "deny"` and wrong-value coverage rejects `warn`.
2. Add or update direct rule tests for `g3rs-deny/wrappers` so the managed `regex` ban wrapper set is empty and any remaining wrapper carveout errors.
3. Update the real deny family baseline in `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/baseline.rs` to match the stricter defaults.
4. Update any shared deny test fixtures in `test_support.rs` that still encode the old wrapper carveout.
5. Verify the deny family workspace tests pass, then verify `guardrail3-rs validate --path packages/rs/clippy/g3rs-clippy-config-checks --family deny` is clean.

# Key Decisions
- Fix the family baseline, not the package. The stale behavior is centralized in `baseline.rs` and shared test fixtures.
- Keep the change narrow. Do not change other deny defaults in this slice.
- Treat this as a bug fix: the baseline should not encode pre-approved escape hatches unless the current package graph proves they are necessary.

# Files To Modify
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/baseline.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/advisories/rs_deny_config_02_advisories_baseline/rule_tests/golden.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/advisories/rs_deny_config_02_advisories_baseline/rule_tests/wrong_values.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers_tests/golden.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers_tests/managed_wrappers.rs`

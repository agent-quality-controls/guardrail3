# Goal
Allow a package to add a narrow local wrapper exception for a centrally managed banned crate without hard-failing the package. The exception should stay visible as a warning instead of an error.

# Approach
1. Change the direct `g3rs-deny/wrappers` tests for managed wrapper additions to expect a warning message instead of an error.
2. Run the targeted tests to prove the current rule is too strict.
3. Update `rs_deny_config_27_wrappers.rs` so managed wrapper changes emit warnings, not errors.
4. Verify the deny workspace tests pass.
5. Re-run the live `tree-sitter` package experiment and confirm:
   - `cargo deny` passes
   - `--family deny` shows a warning instead of an error
   - the remaining hard failure is `deps` because `tree-sitter` is not allowlisted yet

# Key Decisions
- Keep the family baseline strict by default.
- Only soften the response for package-local managed wrapper additions.
- Keep malformed wrapper entries as errors.

# Files To Modify
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers_tests/managed_wrappers.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers_tests/project_specific_wrappers.rs`

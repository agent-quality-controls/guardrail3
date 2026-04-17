Summary
- Cleaned `packages/rs/test/g3rs-test-config-checks` to the current internal package shape and brought it to `No findings.`
- Grouped the runtime under `nextest` and `mutants`, removed the old shared `test_helpers` file, and moved each rule sidecar onto local helpers plus an owned assertions module.

Decisions made
- Fixed the runtime structural-cap error by grouping the rule directories into `nextest` and `mutants` instead of waiving `RS-ARCH-FILETREE-07`.
- Switched runtime off the local `crates/types` facade onto `g3rs-test-types` directly so the cleaned package matches the other modernized check packages.
- Kept the assertions surface minimal but proof-bearing: one owned `rule.rs` per runtime rule, each directly inspecting `G3CheckResult` fields.
- Marked the facade and member crates non-publishable and added the standard root policy files instead of preserving the old publishable shell.

Key files for context
- `packages/rs/test/g3rs-test-config-checks/Cargo.toml`
- `packages/rs/test/g3rs-test-config-checks/guardrail3-rs.toml`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/nextest/mod.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/mutants/mod.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/nextest/rs_test_config_09_nextest_timeouts/rule_tests/helpers.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/assertions/src/nextest/rs_test_config_09_nextest_timeouts/rule.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/assertions/src/mutants/rs_test_config_15_mutants_config_sane/rule.rs`

Next steps
- Continue with `packages/rs/test/g3rs-test-file-tree-checks`, then `packages/rs/test/g3rs-test-ingestion`, then `packages/rs/test/g3rs-test-source-checks`.
- After the `test` family is clean, move on to the remaining `topology` roots.

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations::{
    self as assertions, Severity,
};
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn errors_and_warns_when_managed_test_relaxation_keys_are_missing() {
    let tree = root_workspace_tree("");
    let results = run_for_tests(&tree, "clippy.toml");

    assertions::assert_messages(
        &results,
        &[
            (
                Severity::Warn,
                "clippy test relaxation enabled missing",
                "`allow-dbg-in-tests` must be set explicitly to `false`. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Warn,
                "clippy test relaxation enabled missing",
                "`allow-print-in-tests` must be set explicitly to `false`. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Error,
                "clippy test expect policy misconfigured missing",
                "`allow-expect-in-tests` must be set explicitly to `true`. Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
            ),
            (
                Severity::Error,
                "clippy test panic relaxation enabled missing",
                "`allow-panic-in-tests` must be set explicitly to `false`. panic!() must remain banned in tests.",
            ),
            (
                Severity::Error,
                "clippy test unwrap relaxation enabled missing",
                "`allow-unwrap-in-tests` must be set explicitly to `false`. unwrap() must remain banned in tests.",
            ),
        ],
        "clippy.toml",
    );
}

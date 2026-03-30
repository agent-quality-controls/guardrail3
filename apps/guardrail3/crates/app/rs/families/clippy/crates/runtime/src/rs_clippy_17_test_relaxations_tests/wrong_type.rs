use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations::{
    self as assertions, Severity,
};
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn errors_and_warns_when_managed_test_relaxation_keys_have_wrong_types() {
    let tree = root_workspace_tree(
        r#"
allow-dbg-in-tests = "no"
allow-print-in-tests = 1
allow-expect-in-tests = []
allow-panic-in-tests = { nope = true }
allow-unwrap-in-tests = 3.14
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");

    assertions::assert_messages(
        &results,
        &[
            (
                Severity::Warn,
                "clippy test relaxation enabled wrong type",
                "`allow-dbg-in-tests` must be a bool with value `false`, found string. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Warn,
                "clippy test relaxation enabled wrong type",
                "`allow-print-in-tests` must be a bool with value `false`, found integer. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Error,
                "clippy test expect policy misconfigured wrong type",
                "`allow-expect-in-tests` must be a bool with value `true`, found array. Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
            ),
            (
                Severity::Error,
                "clippy test panic relaxation enabled wrong type",
                "`allow-panic-in-tests` must be a bool with value `false`, found table. panic!() must remain banned in tests.",
            ),
            (
                Severity::Error,
                "clippy test unwrap relaxation enabled wrong type",
                "`allow-unwrap-in-tests` must be a bool with value `false`, found float. unwrap() must remain banned in tests.",
            ),
        ],
        "clippy.toml",
    );
}

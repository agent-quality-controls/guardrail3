use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_for_each_enabled_test_relaxation() {
    let tree = root_workspace_tree(
        r#"
allow-dbg-in-tests = true
allow-expect-in-tests = false
allow-panic-in-tests = true
allow-print-in-tests = true
allow-unwrap-in-tests = true
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_messages(
        &results,
        &[
            (
                assertions::Severity::Warn,
                "clippy test relaxation enabled",
                "`allow-dbg-in-tests = true` relaxes test output discipline.",
            ),
            (
                assertions::Severity::Error,
                "clippy test expect policy misconfigured",
                "`allow-expect-in-tests` must be `true` so tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
            ),
            (
                assertions::Severity::Error,
                "clippy test panic relaxation enabled",
                "`allow-panic-in-tests` must stay `false` so `panic!()` remains banned in tests.",
            ),
            (
                assertions::Severity::Warn,
                "clippy test relaxation enabled",
                "`allow-print-in-tests = true` relaxes test output discipline.",
            ),
            (
                assertions::Severity::Error,
                "clippy test unwrap relaxation enabled",
                "`allow-unwrap-in-tests` must stay `false` so `unwrap()` remains banned in tests.",
            ),
        ],
        "clippy.toml",
    );
}

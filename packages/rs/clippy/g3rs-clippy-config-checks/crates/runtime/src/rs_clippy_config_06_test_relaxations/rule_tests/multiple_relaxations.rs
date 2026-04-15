use g3rs_clippy_config_checks_assertions::rs_clippy_config_06_test_relaxations::rule as assertions;
use super::helpers::run_check;

#[test]
fn flags_wrong_test_relaxation_values() {
    let results = run_check(
        r#"
allow-dbg-in-tests = true
allow-expect-in-tests = false
allow-panic-in-tests = true
allow-print-in-tests = true
allow-unwrap-in-tests = true
"#,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "clippy test relaxation enabled",
                "`allow-dbg-in-tests` must be `false`; found `true`. Tests should stay quiet and deterministic.",
                "clippy.toml",
                false,
            ),
            assertions::warn(
                "clippy test relaxation enabled",
                "`allow-print-in-tests` must be `false`; found `true`. Tests should stay quiet and deterministic.",
                "clippy.toml",
                false,
            ),
            assertions::error(
                "clippy test expect policy misconfigured",
                "`allow-expect-in-tests` must be `true`; found `false`. Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
                "clippy.toml",
                false,
            ),
            assertions::error(
                "clippy test panic relaxation enabled",
                "`allow-panic-in-tests` must be `false`; found `true`. panic!() must remain banned in tests.",
                "clippy.toml",
                false,
            ),
            assertions::error(
                "clippy test unwrap relaxation enabled",
                "`allow-unwrap-in-tests` must be `false`; found `true`. unwrap() must remain banned in tests.",
                "clippy.toml",
                false,
            ),
        ],
    );
}

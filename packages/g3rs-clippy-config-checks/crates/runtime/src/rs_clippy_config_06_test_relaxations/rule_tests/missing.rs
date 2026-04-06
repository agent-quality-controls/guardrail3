use g3rs_clippy_config_checks_assertions::rs_clippy_config_06_test_relaxations as assertions;

use super::helpers::run_check;

#[test]
fn errors_and_warns_when_test_relaxation_keys_are_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "clippy test relaxation enabled missing",
                "`allow-dbg-in-tests` must be set explicitly to `false`. Tests should stay quiet and deterministic.",
                "clippy.toml",
                false,
            ),
            assertions::warn(
                "clippy test relaxation enabled missing",
                "`allow-print-in-tests` must be set explicitly to `false`. Tests should stay quiet and deterministic.",
                "clippy.toml",
                false,
            ),
            assertions::error(
                "clippy test expect policy misconfigured missing",
                "`allow-expect-in-tests` must be set explicitly to `true`. Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
                "clippy.toml",
                false,
            ),
            assertions::error(
                "clippy test panic relaxation enabled missing",
                "`allow-panic-in-tests` must be set explicitly to `false`. panic!() must remain banned in tests.",
                "clippy.toml",
                false,
            ),
            assertions::error(
                "clippy test unwrap relaxation enabled missing",
                "`allow-unwrap-in-tests` must be set explicitly to `false`. unwrap() must remain banned in tests.",
                "clippy.toml",
                false,
            ),
        ],
    );
}

use guardrail3_domain_modules::clippy::{
    ALLOW_DBG_IN_TESTS, ALLOW_EXPECT_IN_TESTS, ALLOW_PANIC_IN_TESTS, ALLOW_PRINT_IN_TESTS,
    ALLOW_UNWRAP_IN_TESTS,
};
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_baseline_keeps_test_relaxation_policy_exact() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");

    assert_eq!(
        parsed
            .get("allow-dbg-in-tests")
            .and_then(toml::Value::as_bool),
        Some(ALLOW_DBG_IN_TESTS),
        "unexpected canonical value for allow-dbg-in-tests",
    );
    assert_eq!(
        parsed
            .get("allow-expect-in-tests")
            .and_then(toml::Value::as_bool),
        Some(ALLOW_EXPECT_IN_TESTS),
        "unexpected canonical value for allow-expect-in-tests",
    );
    assert_eq!(
        parsed
            .get("allow-panic-in-tests")
            .and_then(toml::Value::as_bool),
        Some(ALLOW_PANIC_IN_TESTS),
        "unexpected canonical value for allow-panic-in-tests",
    );
    assert_eq!(
        parsed
            .get("allow-print-in-tests")
            .and_then(toml::Value::as_bool),
        Some(ALLOW_PRINT_IN_TESTS),
        "unexpected canonical value for allow-print-in-tests",
    );
    assert_eq!(
        parsed
            .get("allow-unwrap-in-tests")
            .and_then(toml::Value::as_bool),
        Some(ALLOW_UNWRAP_IN_TESTS),
        "unexpected canonical value for allow-unwrap-in-tests",
    );
}

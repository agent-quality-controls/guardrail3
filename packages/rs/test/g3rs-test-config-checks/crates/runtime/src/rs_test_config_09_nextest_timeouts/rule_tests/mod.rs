use g3rs_test_config_checks_assertions::common::assert_has_result;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_missing_nextest_when_async_surface_is_active() {
    let mut input = crate::test_helpers::input();
    input.has_tests = true;
    input.has_tokio_tests = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-09",
        G3Severity::Error,
        "nextest config missing",
        ".config/nextest.toml",
    );
}

#[test]
fn reports_configured_timeouts_as_inventory() {
    let mut input = crate::test_helpers::input();
    input.has_tests = true;
    input.tokio_dependency_present = true;
    let input = crate::test_helpers::with_nextest(
        input,
        "[profile.default]\nslow-timeout = \"60s\"\nleak-timeout = \"100ms\"\n",
    );

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-09",
        G3Severity::Info,
        "nextest timeouts configured",
        ".config/nextest.toml",
    );
}

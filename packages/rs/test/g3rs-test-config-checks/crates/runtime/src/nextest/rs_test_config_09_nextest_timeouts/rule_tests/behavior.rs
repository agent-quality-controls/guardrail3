use g3rs_test_config_checks_assertions::nextest::rs_test_config_09_nextest_timeouts::rule as assertions;

use super::helpers;

#[test]
fn reports_missing_nextest_when_async_surface_is_active() {
    let mut input = helpers::input();
    input.has_tests = true;
    input.has_tokio_tests = true;

    let results = helpers::run(&input);

    assertions::assert_missing(&results);
}

#[test]
fn reports_configured_timeouts_as_inventory() {
    let mut input = helpers::input();
    input.has_tests = true;
    input.tokio_dependency_present = true;
    let input = helpers::with_nextest(
        input,
        "[profile.default]\nslow-timeout = \"60s\"\nleak-timeout = \"100ms\"\n",
    );

    let results = helpers::run(&input);

    assertions::assert_configured(&results);
}

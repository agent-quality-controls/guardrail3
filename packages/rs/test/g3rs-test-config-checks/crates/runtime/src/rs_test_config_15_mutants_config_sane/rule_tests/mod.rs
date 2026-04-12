use g3rs_test_config_checks_assertions::common::assert_has_result;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_exclude_everything_pattern() {
    let input = crate::test_helpers::with_mutants(
        crate::test_helpers::input(),
        "exclude_re = [\".*\"]\n",
    );

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-15",
        G3Severity::Error,
        "mutants config excludes everything",
        ".cargo/mutants.toml",
    );
}

#[test]
fn reports_low_timeout_multiplier() {
    let input = crate::test_helpers::with_mutants(
        crate::test_helpers::input(),
        "timeout_multiplier = 0.5\n",
    );

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-15",
        G3Severity::Error,
        "mutants timeout multiplier too low",
        ".cargo/mutants.toml",
    );
}

#[test]
fn inventories_sane_config() {
    let input = crate::test_helpers::with_mutants(
        crate::test_helpers::input(),
        "timeout_multiplier = 2.0\nexclude_re = [\"^tests/\"]\n",
    );

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-15",
        G3Severity::Info,
        "mutants config looks sane",
        ".cargo/mutants.toml",
    );
}

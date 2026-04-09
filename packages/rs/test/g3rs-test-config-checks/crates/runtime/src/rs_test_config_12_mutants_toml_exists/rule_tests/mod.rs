use g3rs_test_config_checks_assertions::common::assert_has_result;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_missing_mutants_file() {
    let mut input = crate::test_helpers::input();
    input.mutation_hook_active = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-12",
        G3Severity::Error,
        "mutants config missing",
        ".cargo/mutants.toml",
    );
}

#[test]
fn reports_present_mutants_file_as_inventory() {
    let input = crate::test_helpers::with_mutants(
        crate::test_helpers::input(),
        "timeout_multiplier = 2.0\n",
    );

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-12",
        G3Severity::Info,
        "mutants config exists",
        ".cargo/mutants.toml",
    );
}

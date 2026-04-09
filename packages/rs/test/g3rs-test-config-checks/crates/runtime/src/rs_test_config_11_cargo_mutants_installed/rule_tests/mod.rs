use g3rs_test_config_checks_assertions::common::assert_has_result;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_missing_tool() {
    let mut input = crate::test_helpers::input();
    input.mutation_hook_active = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-11",
        G3Severity::Error,
        "cargo-mutants missing",
        "Cargo.toml",
    );
}

#[test]
fn reports_present_tool_as_inventory() {
    let mut input = crate::test_helpers::input();
    input.mutation_hook_active = true;
    input.cargo_mutants_installed = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-11",
        G3Severity::Info,
        "cargo-mutants installed",
        "Cargo.toml",
    );
}

use g3rs_test_config_checks_assertions::common::assert_has_result;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_missing_hook_step() {
    let mut input = crate::test_helpers::input();
    input.mutants_exists = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-14",
        G3Severity::Error,
        "mutation hook step missing",
        "Cargo.toml",
    );
}

#[test]
fn inventories_present_hook_files() {
    let mut input = crate::test_helpers::input();
    input.mutation_hook_active = true;
    input.mutation_hook_files = vec![".githooks/pre-commit".to_owned()];

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-14",
        G3Severity::Info,
        "mutation hook step present",
        ".githooks/pre-commit",
    );
}

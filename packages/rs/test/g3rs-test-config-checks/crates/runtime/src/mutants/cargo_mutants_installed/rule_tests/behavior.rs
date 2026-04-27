use g3rs_test_config_checks_assertions::mutants::cargo_mutants_installed::rule as assertions;

use super::helpers;

#[test]
fn reports_missing_tool() {
    let mut input = helpers::input();
    input.mutation_hook_active = true;

    let results = helpers::run(&input);

    assertions::assert_missing(&results);
}

#[test]
fn reports_present_tool_as_inventory() {
    let mut input = helpers::input();
    input.mutation_hook_active = true;
    input.cargo_mutants_installed = true;

    let results = helpers::run(&input);

    assertions::assert_present(&results);
}

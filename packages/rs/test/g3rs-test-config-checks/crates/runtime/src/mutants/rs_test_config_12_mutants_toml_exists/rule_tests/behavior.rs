use g3rs_test_config_checks_assertions::mutants::rs_test_config_12_mutants_toml_exists::rule as assertions;

use super::helpers;

#[test]
fn reports_missing_mutants_file() {
    let mut input = helpers::input();
    input.mutation_hook_active = true;

    let results = helpers::run(&input);

    assertions::assert_missing(&results);
}

#[test]
fn reports_present_mutants_file_as_inventory() {
    let input = helpers::with_mutants(helpers::input(), "timeout_multiplier = 2.0\n");

    let results = helpers::run(&input);

    assertions::assert_present(&results);
}

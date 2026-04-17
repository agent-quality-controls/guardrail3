use g3rs_test_config_checks_assertions::mutants::rs_test_config_14_mutation_hook_present::rule as assertions;

use super::helpers;

#[test]
fn reports_missing_hook_step() {
    let mut input = helpers::input();
    input.mutants_exists = true;

    let results = helpers::run(&input);

    assertions::assert_missing(&results);
}

#[test]
fn inventories_present_hook_files() {
    let mut input = helpers::input();
    input.mutation_hook_active = true;
    input.mutation_hook_files = vec![".githooks/pre-commit".to_owned()];

    let results = helpers::run(&input);

    assertions::assert_present(&results);
}

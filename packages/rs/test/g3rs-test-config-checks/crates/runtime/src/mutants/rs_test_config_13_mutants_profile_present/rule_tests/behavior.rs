use g3rs_test_config_checks_assertions::mutants::rs_test_config_13_mutants_profile_present::rule as assertions;

use super::helpers;

#[test]
fn reports_missing_mutants_profile() {
    let mut input = helpers::input();
    input.mutants_exists = true;

    let results = helpers::run(&input);

    assertions::assert_missing(&results);
}

#[test]
fn reports_mutants_profile_as_inventory() {
    let input = helpers::with_mutants_profile(helpers::input());

    let results = helpers::run(&input);

    assertions::assert_present(&results);
}

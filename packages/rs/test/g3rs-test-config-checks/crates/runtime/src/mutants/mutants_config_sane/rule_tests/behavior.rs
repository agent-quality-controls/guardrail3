use g3rs_test_config_checks_assertions::mutants::mutants_config_sane::rule as assertions;

use super::helpers;

#[test]
fn reports_exclude_everything_pattern() {
    let input = helpers::with_mutants(helpers::input(), "exclude_re = [\".*\"]\n");

    let results = helpers::run(&input);

    assertions::assert_excludes_everything(&results);
}

#[test]
fn reports_low_timeout_multiplier() {
    let input = helpers::with_mutants(helpers::input(), "timeout_multiplier = 0.5\n");

    let results = helpers::run(&input);

    assertions::assert_timeout_too_low(&results);
}

#[test]
fn inventories_sane_config() {
    let input = helpers::with_mutants(
        helpers::input(),
        "timeout_multiplier = 2.0\nexclude_re = [\"^tests/\"]\n",
    );

    let results = helpers::run(&input);

    assertions::assert_sane(&results);
}

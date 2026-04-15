use g3rs_cargo_config_checks_assertions::rs_cargo_config_04_priority_order::rule as assertions;
use super::helpers::run_check;

#[test]
fn inventories_when_specific_deny_lints_do_not_use_negative_priority() {
    let results = run_check(include_str!("fixtures/golden_workspace.toml"));

    assertions::assert_has_info(&results, "specific lint priorities are safe", true);
}

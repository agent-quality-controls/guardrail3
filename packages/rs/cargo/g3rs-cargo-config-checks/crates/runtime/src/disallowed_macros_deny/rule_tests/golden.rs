use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::disallowed_macros_deny::rule as assertions;

#[test]
fn inventories_when_disallowed_macros_is_denied() {
    let results = run_check(include_str!("fixtures/golden_workspace.toml"));

    assertions::assert_has_info(&results, "disallowed macros lint enforced", true);
}

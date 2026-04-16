use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::rs_cargo_config_02_lint_levels::rule as assertions;

#[test]
fn inventories_when_lint_levels_match_policy() {
    let results = run_check(include_str!("fixtures/golden_workspace.toml"));

    assertions::assert_has_info(&results, "lint levels match policy", true);
}

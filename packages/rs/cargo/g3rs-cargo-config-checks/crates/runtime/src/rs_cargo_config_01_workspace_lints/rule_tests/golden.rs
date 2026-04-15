use g3rs_cargo_config_checks_assertions::rs_cargo_config_01_workspace_lints::rule as assertions;
use super::helpers::run_check;

#[test]
fn inventories_when_required_lint_tables_are_present() {
    let results = run_check(include_str!("fixtures/golden_workspace.toml"));

    assertions::assert_has_info(&results, "workspace lint tables present", true);
}

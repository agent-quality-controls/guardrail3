use g3rs_cargo_config_checks_assertions::rs_cargo_config_02_lint_levels::rule as assertions;
use super::helpers::run_check;

#[test]
fn inventories_when_hybrid_root_falls_back_to_package_lint_levels() {
    let results = run_check(include_str!("fixtures/golden_hybrid_package.toml"));

    assertions::assert_has_info(&results, "lint levels match policy", true);
}

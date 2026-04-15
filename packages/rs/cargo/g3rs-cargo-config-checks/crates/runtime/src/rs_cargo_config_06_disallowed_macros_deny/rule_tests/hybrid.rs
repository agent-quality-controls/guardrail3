use g3rs_cargo_config_checks_assertions::rs_cargo_config_06_disallowed_macros_deny::rule as assertions;
use super::helpers::run_check;

#[test]
fn inventories_when_hybrid_root_falls_back_to_package_disallowed_macros_lint() {
    let results = run_check(include_str!("fixtures/golden_hybrid_package.toml"));

    assertions::assert_has_info(&results, "disallowed macros lint enforced", true);
}

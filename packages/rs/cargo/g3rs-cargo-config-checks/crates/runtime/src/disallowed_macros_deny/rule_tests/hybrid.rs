use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::disallowed_macros_deny::rule as assertions;

#[test]
fn inventories_when_hybrid_root_falls_back_to_package_disallowed_macros_lint() {
    let results = run_check(include_str!("fixtures/golden_hybrid_package.toml"));

    assertions::assert_has_info(&results, "disallowed macros lint enforced", true);
}

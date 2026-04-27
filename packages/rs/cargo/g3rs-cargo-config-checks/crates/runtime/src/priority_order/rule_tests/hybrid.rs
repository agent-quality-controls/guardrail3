use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::priority_order::rule as assertions;

#[test]
fn inventories_when_hybrid_root_falls_back_to_package_clippy_priorities() {
    let results = run_check(include_str!("fixtures/golden_hybrid_package.toml"));

    assertions::assert_has_info(&results, "specific lint priorities are safe", true);
}

use g3rs_clippy_config_checks_assertions::rs_clippy_config_08_type_complexity_threshold::rule as assertions;
use super::helpers::run_check;

#[test]
fn inventories_when_type_complexity_threshold_matches_baseline() {
    let results = run_check("type-complexity-threshold = 75\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "type-complexity-threshold correct",
            "type-complexity-threshold = 75",
            "clippy.toml",
            true,
        )],
    );
}

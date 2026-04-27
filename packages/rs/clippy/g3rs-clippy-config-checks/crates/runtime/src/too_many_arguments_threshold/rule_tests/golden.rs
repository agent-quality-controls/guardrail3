use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::too_many_arguments_threshold::rule as assertions;

#[test]
fn inventories_when_too_many_arguments_threshold_matches_baseline() {
    let results = run_check("too-many-arguments-threshold = 7\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "too-many-arguments-threshold correct",
            "too-many-arguments-threshold = 7",
            "clippy.toml",
            true,
        )],
    );
}

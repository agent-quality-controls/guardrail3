use g3rs_clippy_config_checks_assertions::rs_clippy_config_05_excessive_nesting_threshold as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_excessive_nesting_threshold_matches_baseline() {
    let results = run_check("excessive-nesting-threshold = 4\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "excessive-nesting-threshold correct",
            "excessive-nesting-threshold = 4",
            "clippy.toml",
            true,
        )],
    );
}

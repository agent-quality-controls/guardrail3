use g3rs_clippy_config_checks_assertions::rs_clippy_config_03_too_many_lines_threshold as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_too_many_lines_threshold_matches_baseline() {
    let results = run_check("too-many-lines-threshold = 75\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "too-many-lines-threshold correct",
            "too-many-lines-threshold = 75",
            "clippy.toml",
            true,
        )],
    );
}

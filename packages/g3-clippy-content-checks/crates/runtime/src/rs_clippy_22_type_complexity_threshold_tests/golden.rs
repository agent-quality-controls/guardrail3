use g3_clippy_content_checks_assertions::rs_clippy_22_type_complexity_threshold as assertions;

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

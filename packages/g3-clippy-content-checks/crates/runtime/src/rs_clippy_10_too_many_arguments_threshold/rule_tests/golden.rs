use g3_clippy_content_checks_assertions::rs_clippy_10_too_many_arguments_threshold as assertions;

use super::helpers::run_check;

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

use g3_clippy_content_checks_assertions::rs_clippy_21_cognitive_complexity_threshold as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_cognitive_complexity_threshold_matches_baseline() {
    let results = run_check("cognitive-complexity-threshold = 15\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "cognitive-complexity-threshold correct",
            "cognitive-complexity-threshold = 15",
            "clippy.toml",
            true,
        )],
    );
}

use super::assertions;

use super::helpers::run_check;

#[test]
fn errors_when_cognitive_complexity_threshold_has_the_wrong_value() {
    let results = run_check("cognitive-complexity-threshold = 12\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "cognitive-complexity-threshold wrong value",
            "Expected 15, got 12. Set `cognitive-complexity-threshold = 15` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

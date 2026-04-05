use g3_clippy_content_checks_assertions::rs_clippy_22_type_complexity_threshold as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_type_complexity_threshold_has_the_wrong_value() {
    let results = run_check("type-complexity-threshold = 100\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "type-complexity-threshold wrong value",
            "Expected 75, got 100. Set `type-complexity-threshold = 75` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

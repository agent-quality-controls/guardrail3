use g3rs_clippy_config_checks_assertions::rs_clippy_config_03_too_many_lines_threshold as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_too_many_lines_threshold_has_the_wrong_value() {
    let results = run_check("too-many-lines-threshold = 120\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "too-many-lines-threshold wrong value",
            "Expected 75, got 120. Set `too-many-lines-threshold = 75` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

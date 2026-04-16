use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_03_too_many_lines_threshold::rule as assertions;

#[test]
fn errors_when_too_many_lines_threshold_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "too-many-lines-threshold missing",
            "Add `too-many-lines-threshold = 75` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

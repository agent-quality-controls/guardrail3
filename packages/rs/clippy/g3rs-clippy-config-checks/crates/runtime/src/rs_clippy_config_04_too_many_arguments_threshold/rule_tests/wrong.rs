use g3rs_clippy_config_checks_assertions::rs_clippy_config_04_too_many_arguments_threshold::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_too_many_arguments_threshold_has_the_wrong_value() {
    let results = run_check("too-many-arguments-threshold = 9\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "too-many-arguments-threshold wrong value",
            "Expected 7, got 9. Set `too-many-arguments-threshold = 7` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

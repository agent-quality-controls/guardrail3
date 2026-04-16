use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_04_too_many_arguments_threshold::rule as assertions;

#[test]
fn errors_when_too_many_arguments_threshold_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "too-many-arguments-threshold missing",
            "Add `too-many-arguments-threshold = 7` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

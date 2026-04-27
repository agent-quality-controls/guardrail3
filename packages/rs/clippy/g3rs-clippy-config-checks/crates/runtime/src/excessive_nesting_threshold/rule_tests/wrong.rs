use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::excessive_nesting_threshold::rule as assertions;

#[test]
fn errors_when_excessive_nesting_threshold_has_the_wrong_value() {
    let results = run_check("excessive-nesting-threshold = 6\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "excessive-nesting-threshold wrong value",
            "Expected 4, got 6. Set `excessive-nesting-threshold = 4` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

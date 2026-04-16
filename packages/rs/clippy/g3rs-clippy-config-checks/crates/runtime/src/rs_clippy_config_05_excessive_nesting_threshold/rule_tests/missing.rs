use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_05_excessive_nesting_threshold::rule as assertions;

#[test]
fn errors_when_excessive_nesting_threshold_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "excessive-nesting-threshold missing",
            "Add `excessive-nesting-threshold = 4` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

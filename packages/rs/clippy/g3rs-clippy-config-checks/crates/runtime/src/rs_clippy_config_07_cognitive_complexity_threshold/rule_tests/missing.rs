use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_07_cognitive_complexity_threshold::rule as assertions;

#[test]
fn errors_when_cognitive_complexity_threshold_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "cognitive-complexity-threshold missing",
            "Add `cognitive-complexity-threshold = 15` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

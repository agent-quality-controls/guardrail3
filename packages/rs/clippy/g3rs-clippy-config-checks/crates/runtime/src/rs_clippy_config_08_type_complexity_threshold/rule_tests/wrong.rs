use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_08_type_complexity_threshold::rule as assertions;

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

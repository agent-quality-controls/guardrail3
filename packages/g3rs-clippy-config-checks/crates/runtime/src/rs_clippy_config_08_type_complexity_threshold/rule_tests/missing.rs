use g3rs_clippy_config_checks_assertions::rs_clippy_config_08_type_complexity_threshold as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_type_complexity_threshold_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "type-complexity-threshold missing",
            "Add `type-complexity-threshold = 75` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

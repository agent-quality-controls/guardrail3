use g3_clippy_content_checks_assertions::rs_clippy_22_type_complexity_threshold as assertions;

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

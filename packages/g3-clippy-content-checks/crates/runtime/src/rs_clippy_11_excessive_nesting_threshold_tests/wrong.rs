use g3_clippy_content_checks_assertions::rs_clippy_11_excessive_nesting_threshold as assertions;

use super::helpers::run_check;

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

use super::assertions;

use super::helpers::run_check;

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

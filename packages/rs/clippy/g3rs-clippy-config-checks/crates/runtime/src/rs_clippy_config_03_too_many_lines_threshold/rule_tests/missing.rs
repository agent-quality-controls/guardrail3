use super::assertions;

use super::helpers::run_check;

#[test]
fn errors_when_too_many_lines_threshold_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "too-many-lines-threshold missing",
            "Add `too-many-lines-threshold = 75` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

use g3rs_release_source_checks_assertions::rs_release_source_02_input_failures as assertions;

use super::helpers;

#[test]
fn reports_input_failure() {
    let results = helpers::check("README.md", "Failed to read README");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "failed to read release source input",
            "Failed to read README",
            "README.md",
            false,
        )],
    );
}

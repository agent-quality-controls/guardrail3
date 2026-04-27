use g3rs_release_filetree_checks_assertions::input_failures as assertions;
use g3rs_release_types::G3RsReleaseInputFailure;

#[test]
fn reports_input_failure() {
    let mut results = Vec::new();
    super::super::check(
        &G3RsReleaseInputFailure {
            rel_path: "README.md".to_owned(),
            message: "Failed to read README".to_owned(),
        },
        &mut results,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "failed to read release filetree input",
            "Failed to read README",
            "README.md",
            false,
        )],
    );
}

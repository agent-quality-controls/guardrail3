use g3rs_deny_filetree_checks_assertions::rs_deny_filetree_01_coverage as assertions;
use test_support::input;

#[test]
fn errors_when_no_root_deny_config_exists() {
    let input = input(None, Vec::new(), Vec::new());
    let mut results = Vec::new();

    crate::rs_deny_filetree_01_coverage::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error_no_file(
            "workspace root uncovered by deny config",
            "workspace root `.` is not covered by any allowed deny config.",
            false,
        )],
    );
}

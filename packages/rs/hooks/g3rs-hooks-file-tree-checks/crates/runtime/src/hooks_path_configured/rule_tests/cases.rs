use g3rs_hooks_file_tree_checks_assertions::hooks_path_configured::rule as assertions;

use super::super::check;

#[test]
fn reports_missing_hooks_path() {
    let mut results = Vec::new();
    check(None, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "core.hooksPath not configured",
            "git config core.hooksPath does not resolve to `.githooks`.",
            None,
            false,
        ),
    );
}

#[test]
fn inventories_expected_hooks_path() {
    let mut results = Vec::new();
    check(Some(".githooks"), &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "core.hooksPath configured",
            "git config core.hooksPath points to `.githooks`.",
            None,
            true,
        ),
    );
}

#[test]
fn treats_empty_hooks_path_as_wrong_value() {
    let mut results = Vec::new();
    check(Some(""), &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "core.hooksPath has wrong value",
            "Expected `.githooks`, got ``.",
            None,
            false,
        ),
    );
}

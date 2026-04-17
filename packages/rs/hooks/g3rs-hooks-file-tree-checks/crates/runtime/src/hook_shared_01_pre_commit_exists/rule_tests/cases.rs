use g3rs_hooks_file_tree_checks_assertions::hook_shared_01_pre_commit_exists::rule as assertions;

use super::helpers::script;
use super::super::check;

#[test]
fn reports_missing_hook() {
    let mut results = Vec::new();
    check(None, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "pre-commit hook missing",
            "Expected a cached `.githooks/pre-commit` or `hooks/pre-commit` hook.",
            Some(".githooks/pre-commit"),
            false,
        ),
    );
}

#[test]
fn inventories_present_hook() {
    let mut results = Vec::new();
    check(Some(&script(".githooks/pre-commit", 2, 24, Some(true))), &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "pre-commit hook exists",
            "Found cached pre-commit hook.",
            Some(".githooks/pre-commit"),
            true,
        ),
    );
}

use g3rs_hooks_file_tree_checks_assertions::pre_commit_executable::rule as assertions;

use super::super::check;
use super::helpers::script;

#[test]
fn reports_non_executable_hook() {
    let mut results = Vec::new();
    check(
        &script(".githooks/pre-commit", 2, 24, Some(false)),
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "pre-commit hook is not executable",
            "Dispatcher hook exists but does not have the executable bit set.",
            Some(".githooks/pre-commit"),
            false,
        ),
    );
}

#[test]
fn inventories_executable_hook() {
    let mut results = Vec::new();
    check(
        &script(".githooks/pre-commit", 2, 24, Some(true)),
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "pre-commit hook is executable",
            "Dispatcher hook has the executable bit set.",
            Some(".githooks/pre-commit"),
            true,
        ),
    );
}

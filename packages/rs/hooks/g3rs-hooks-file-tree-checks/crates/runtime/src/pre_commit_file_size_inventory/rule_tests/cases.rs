use g3rs_hooks_file_tree_checks_assertions::pre_commit_file_size_inventory::rule as assertions;

use super::super::check;
use super::helpers::script;

#[test]
fn inventories_pre_commit_size() {
    let mut results = Vec::new();
    check(
        &script(".githooks/pre-commit", 3, 48, Some(true)),
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "pre-commit file size",
            "48 bytes",
            Some(".githooks/pre-commit"),
            true,
        ),
    );
}

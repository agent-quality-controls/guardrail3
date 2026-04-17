use g3rs_hooks_file_tree_checks_assertions::hook_shared_03_modular_directory_inventory::rule as assertions;

use super::super::check;

#[test]
fn inventories_monolithic_mode() {
    let mut results = Vec::new();
    check(false, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "pre-commit.d directory missing",
            "Hook currently uses a monolithic pre-commit script.",
            Some(".githooks"),
            true,
        ),
    );
}

#[test]
fn inventories_modular_mode() {
    let mut results = Vec::new();
    check(true, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "pre-commit.d directory exists",
            "Hook uses modular pre-commit scripts.",
            Some(".githooks/pre-commit.d"),
            true,
        ),
    );
}

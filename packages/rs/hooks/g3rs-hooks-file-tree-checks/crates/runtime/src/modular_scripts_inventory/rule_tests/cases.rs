use g3rs_hooks_file_tree_checks_assertions::modular_scripts_inventory::rule as assertions;

use super::super::check;
use super::helpers::script;

#[test]
fn inventories_empty_modular_dir() {
    let mut results = Vec::new();
    check(&[], &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "no modular hook scripts",
            "No cached files found in `.githooks/pre-commit.d`.",
            Some(".githooks/pre-commit.d"),
            true,
        ),
    );
}

#[test]
fn inventories_script_names() {
    let mut results = Vec::new();
    check(
        &[
            script(".githooks/pre-commit.d/10-rust.sh", 2, 24, Some(true)),
            script(".githooks/pre-commit.d/20-lock.sh", 1, 16, Some(true)),
        ],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "modular hook scripts inventory",
            ".githooks/pre-commit.d/10-rust.sh, .githooks/pre-commit.d/20-lock.sh",
            Some(".githooks/pre-commit.d"),
            true,
        ),
    );
}

use g3rs_hooks_file_tree_checks_assertions::local_override_inventory::rule as assertions;

use super::super::check;

#[test]
fn inventories_no_overrides() {
    let mut results = Vec::new();
    check(&[], &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "no local hook overrides",
            "No cached override scripts found in `.guardrail3/overrides/pre-commit.d`.",
            Some(".guardrail3/overrides/pre-commit.d"),
            true,
        ),
    );
}

#[test]
fn inventories_override_names() {
    let mut results = Vec::new();
    check(
        &["10-local.sh".to_owned(), "20-debug.sh".to_owned()],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Info,
            "local hook overrides inventory",
            "10-local.sh, 20-debug.sh",
            Some(".guardrail3/overrides/pre-commit.d"),
            true,
        ),
    );
}

use g3rs_hooks_file_tree_checks_assertions::execution_trust::rule as assertions;

use super::super::check;

#[test]
fn inventories_clean_trust_surface() {
    let mut results = Vec::new();
    check(&[], &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Warn,
            "no competing hook systems detected",
            "No obvious alternate hook system or shadowing risk was found.",
            None,
            true,
        ),
    );
}

#[test]
fn reports_competing_hook_surfaces() {
    let mut results = Vec::new();
    check(
        &[
            ".husky/pre-commit".to_owned(),
            ".git/hooks/pre-commit".to_owned(),
        ],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Warn,
            "competing hook system detected",
            "Found alternate hook surfaces that can shadow or confuse hook execution: .husky/pre-commit, .git/hooks/pre-commit",
            None,
            false,
        ),
    );
}

use g3rs_hooks_file_tree_checks_assertions::hook_shared_17_execution_trust as assertions;

#[test]
fn inventories_clean_trust_surface() {
    let mut results = Vec::new();
    crate::hook_shared_17_execution_trust::check(&[], &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("no competing hook systems detected"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_competing_hook_surfaces() {
    let mut results = Vec::new();
    crate::hook_shared_17_execution_trust::check(
        &[".husky/pre-commit".to_owned(), ".git/hooks/pre-commit".to_owned()],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("competing hook system detected"),
            message: Some(
                "Found alternate hook surfaces that can shadow or confuse hook execution: .husky/pre-commit, .git/hooks/pre-commit",
            ),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

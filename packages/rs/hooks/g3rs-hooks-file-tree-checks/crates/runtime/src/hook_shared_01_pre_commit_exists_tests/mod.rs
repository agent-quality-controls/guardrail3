use g3rs_hooks_file_tree_checks_assertions::hook_shared_01_pre_commit_exists as assertions;

#[test]
fn reports_missing_hook() {
    let mut results = Vec::new();
    crate::hook_shared_01_pre_commit_exists::check(None, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit hook missing"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_present_hook() {
    let mut results = Vec::new();
    crate::hook_shared_01_pre_commit_exists::check(
        Some(&crate::test_support::script(".githooks/pre-commit", 2, 24, Some(true))),
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit hook exists"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

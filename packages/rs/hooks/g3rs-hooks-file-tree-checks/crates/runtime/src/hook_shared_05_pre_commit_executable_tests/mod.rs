use g3rs_hooks_file_tree_checks_assertions::hook_shared_05_pre_commit_executable as assertions;

#[test]
fn reports_non_executable_hook() {
    let mut results = Vec::new();
    crate::hook_shared_05_pre_commit_executable::check(
        &crate::test_support::script(".githooks/pre-commit", 2, 24, Some(false)),
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit hook is not executable"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_executable_hook() {
    let mut results = Vec::new();
    crate::hook_shared_05_pre_commit_executable::check(
        &crate::test_support::script(".githooks/pre-commit", 2, 24, Some(true)),
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit hook is executable"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

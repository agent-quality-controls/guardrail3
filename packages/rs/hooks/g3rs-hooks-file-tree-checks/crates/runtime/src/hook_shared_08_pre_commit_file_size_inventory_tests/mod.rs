use g3rs_hooks_file_tree_checks_assertions::hook_shared_08_pre_commit_file_size_inventory as assertions;

#[test]
fn inventories_pre_commit_size() {
    let mut results = Vec::new();
    crate::hook_shared_08_pre_commit_file_size_inventory::check(
        &crate::test_support::script(".githooks/pre-commit", 3, 48, Some(true)),
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit file size"),
            file: Some(".githooks/pre-commit"),
            message: Some("48 bytes"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

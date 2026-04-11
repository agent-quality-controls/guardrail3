use g3rs_hooks_file_tree_checks_assertions::hook_shared_06_script_stats_inventory as assertions;

#[test]
fn inventories_script_stats() {
    let mut results = Vec::new();
    crate::hook_shared_06_script_stats_inventory::check(
        &crate::test_support::script(".githooks/pre-commit", 3, 48, Some(true)),
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit script stats"),
            file: Some(".githooks/pre-commit"),
            message: Some("3 lines, 48 bytes"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

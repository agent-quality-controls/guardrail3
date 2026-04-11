use g3rs_hooks_file_tree_checks_assertions::hook_shared_09_local_override_inventory as assertions;

#[test]
fn inventories_no_overrides() {
    let mut results = Vec::new();
    crate::hook_shared_09_local_override_inventory::check(&[], &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("no local hook overrides"),
            file: Some(".guardrail3/overrides/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_override_names() {
    let mut results = Vec::new();
    crate::hook_shared_09_local_override_inventory::check(
        &[
            "10-local.sh".to_owned(),
            "20-debug.sh".to_owned(),
        ],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("local hook overrides inventory"),
            file: Some(".guardrail3/overrides/pre-commit.d"),
            message: Some("10-local.sh, 20-debug.sh"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

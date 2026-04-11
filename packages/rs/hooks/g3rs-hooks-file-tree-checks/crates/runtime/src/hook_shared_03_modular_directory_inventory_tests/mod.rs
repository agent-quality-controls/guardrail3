use g3rs_hooks_file_tree_checks_assertions::hook_shared_03_modular_directory_inventory as assertions;

#[test]
fn inventories_monolithic_mode() {
    let mut results = Vec::new();
    crate::hook_shared_03_modular_directory_inventory::check(false, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit.d directory missing"),
            file: Some(".githooks"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_modular_mode() {
    let mut results = Vec::new();
    crate::hook_shared_03_modular_directory_inventory::check(true, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("pre-commit.d directory exists"),
            file: Some(".githooks/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

use g3rs_hooks_file_tree_checks_assertions::hook_shared_07_modular_scripts_inventory as assertions;

#[test]
fn inventories_empty_modular_dir() {
    let mut results = Vec::new();
    crate::hook_shared_07_modular_scripts_inventory::check(&[], &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("no modular hook scripts"),
            file: Some(".githooks/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_script_names() {
    let mut results = Vec::new();
    crate::hook_shared_07_modular_scripts_inventory::check(
        &[
            crate::test_support::script(".githooks/pre-commit.d/10-rust.sh", 2, 24, Some(true)),
            crate::test_support::script(".githooks/pre-commit.d/20-lock.sh", 1, 16, Some(true)),
        ],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("modular hook scripts inventory"),
            file: Some(".githooks/pre-commit.d"),
            message: Some(".githooks/pre-commit.d/10-rust.sh, .githooks/pre-commit.d/20-lock.sh"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

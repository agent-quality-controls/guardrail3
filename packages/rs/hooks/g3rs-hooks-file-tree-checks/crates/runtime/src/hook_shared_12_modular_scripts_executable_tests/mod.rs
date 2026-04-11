use g3rs_hooks_file_tree_checks_assertions::hook_shared_12_modular_scripts_executable as assertions;

#[test]
fn reports_non_executable_modular_script() {
    let mut results = Vec::new();
    crate::hook_shared_12_modular_scripts_executable::check(
        &[crate::test_support::script(
            ".githooks/pre-commit.d/10-rust.sh",
            1,
            16,
            Some(false),
        )],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("modular hook script is not executable"),
            file: Some(".githooks/pre-commit.d/10-rust.sh"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_executable_modular_script() {
    let mut results = Vec::new();
    crate::hook_shared_12_modular_scripts_executable::check(
        &[crate::test_support::script(
            ".githooks/pre-commit.d/10-rust.sh",
            1,
            16,
            Some(true),
        )],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("modular hook script is executable"),
            file: Some(".githooks/pre-commit.d/10-rust.sh"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

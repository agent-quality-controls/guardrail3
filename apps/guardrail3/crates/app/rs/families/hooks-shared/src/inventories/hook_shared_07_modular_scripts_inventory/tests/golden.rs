use guardrail3_app_rs_family_hooks_shared_assertions::inventories::hook_shared_07_modular_scripts_inventory as assertions;

use super::run_case;

#[test]
fn inventories_no_modular_scripts() {
    let results = run_case(&[]);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("no modular hook scripts"),
            file: Some(".githooks/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_modular_script_names() {
    let results = run_case(&["10-rust.sh", "20-ts.sh"]);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("modular hook scripts inventory"),
            file: Some(".githooks/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    assertions::assert_message_contains_all(&results, &["10-rust.sh", "20-ts.sh"]);
}

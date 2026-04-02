use guardrail3_app_rs_family_hooks_shared_assertions::inventories::hook_shared_08_pre_commit_file_size_inventory as assertions;

use crate::hook_shared_08_pre_commit_file_size_inventory::check;

#[test]
fn reports_pre_commit_file_size() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", "abcd", &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("pre-commit file size"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(true),
            message: Some("4 bytes"),
            ..Default::default()
        }],
    );
}

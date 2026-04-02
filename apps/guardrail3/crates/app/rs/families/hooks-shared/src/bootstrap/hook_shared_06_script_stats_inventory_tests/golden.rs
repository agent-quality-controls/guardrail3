use guardrail3_app_rs_family_hooks_shared_assertions::bootstrap::hook_shared_06_script_stats_inventory as assertions;

use crate::hook_shared_06_script_stats_inventory::check;

#[test]
fn reports_line_and_byte_counts() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", "line1\nline2\n", &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("pre-commit script stats"),
            file: Some(".githooks/pre-commit"),
            inventory: Some(true),
            message: Some("2 lines, 12 bytes"),
            ..Default::default()
        }],
    );
}

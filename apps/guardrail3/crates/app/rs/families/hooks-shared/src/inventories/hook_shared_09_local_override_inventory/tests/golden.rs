use guardrail3_app_rs_family_hooks_shared_assertions::inventories::hook_shared_09_local_override_inventory as assertions;

use super::check;

#[test]
fn inventories_no_overrides() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("no local hook overrides"),
            file: Some(".guardrail3/overrides/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_override_names() {
    let overrides = vec!["99-local.sh".to_owned(), "20-extra.sh".to_owned()];
    let mut results = Vec::new();
    check(&overrides, &mut results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("local hook overrides inventory"),
            file: Some(".guardrail3/overrides/pre-commit.d"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    assertions::assert_message_contains(&results, "99-local.sh");
}

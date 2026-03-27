use super::{ExpectedRuleResult, assert_rule_results, lockfile_facts, lockfile_input};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_clean_gitignore() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            inventory: Some(true),
            message: Some("No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`."),
            ..ExpectedRuleResult::default()
        }],
    );
}

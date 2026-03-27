use super::{ExpectedRuleResult, assert_rule_results, lockfile_facts, lockfile_input};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_present_cargo_lock() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            inventory: Some(true),
            message: Some("Rust root `.` has `Cargo.lock` committed."),
            ..ExpectedRuleResult::default()
        }],
    );
}

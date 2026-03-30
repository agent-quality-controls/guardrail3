use super::{lockfile_facts, lockfile_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_09_cargo_lock_present as assertions;

#[test]
fn inventories_present_cargo_lock() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            inventory: Some(true),
            message: Some("Rust root `.` has `Cargo.lock` committed."),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}

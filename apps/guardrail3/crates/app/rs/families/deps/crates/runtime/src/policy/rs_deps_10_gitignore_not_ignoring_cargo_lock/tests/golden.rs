use super::{lockfile_facts, lockfile_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_10_gitignore_not_ignoring_cargo_lock as assertions;

#[test]
fn inventories_clean_gitignore() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    super::helpers::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            inventory: Some(true),
            message: Some("No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`."),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}

use super::{coverage_facts, coverage_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_08_library_allowlist_present as assertions;

#[test]
fn inventories_library_allowlist_when_present() {
    let facts = coverage_facts(Some("library"), true);
    let input = coverage_input(&facts, "packages/core/Cargo.toml");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("library allowlist present"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}

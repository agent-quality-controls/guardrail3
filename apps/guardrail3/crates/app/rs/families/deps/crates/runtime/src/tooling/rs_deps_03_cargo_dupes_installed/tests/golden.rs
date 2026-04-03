use super::{tool_facts, tool_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_03_cargo_dupes_installed as assertions;

#[test]
fn inventories_installed_cargo_dupes() {
    let facts = tool_facts("cargo-dupes", true);
    let input = tool_input(&facts, "cargo-dupes");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("cargo-dupes installed"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}

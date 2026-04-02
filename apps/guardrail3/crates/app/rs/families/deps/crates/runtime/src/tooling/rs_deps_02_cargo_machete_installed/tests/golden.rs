use super::{tool_facts, tool_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_02_cargo_machete_installed as assertions;

#[test]
fn inventories_installed_cargo_machete() {
    let facts = tool_facts("cargo-machete", true);
    let input = tool_input(&facts, "cargo-machete");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("cargo-machete installed"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}

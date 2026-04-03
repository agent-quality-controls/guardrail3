use super::{tool_facts, tool_input};
use guardrail3_app_rs_family_deps_assertions::rs_deps_01_cargo_deny_installed as assertions;

#[test]
fn inventories_installed_cargo_deny() {
    let facts = tool_facts("cargo-deny", true);
    let input = tool_input(&facts, "cargo-deny");
    let mut results = Vec::new();

    super::helpers::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("cargo-deny installed"),
            inventory: Some(true),
            ..assertions::ExpectedRuleResult::default()
        }],
    );
}

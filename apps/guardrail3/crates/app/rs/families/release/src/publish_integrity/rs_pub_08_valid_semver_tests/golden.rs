use super::super::check;
use super::super::{crate_facts, crate_input};
use guardrail3_app_rs_family_release_assertions::rs_pub_08_valid_semver as assertions;

#[test]
fn inventories_valid_semver() {
    let valid = crate_facts("x");
    let valid_input = crate_input(&valid);
    let mut valid_results = Vec::new();
    check(&valid_input, &mut valid_results);

    assert!(!assertions::findings(&valid_results).is_empty());
    assertions::assert_rule_results(
        &valid_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title_contains: Some("valid semver"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("1.2.3"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_valid_workspace_version_inheritance() {
    let mut workspace = crate_facts("x");
    workspace.workspace_version = true;
    let workspace_input = crate_input(&workspace);
    let mut workspace_results = Vec::new();
    check(&workspace_input, &mut workspace_results);

    assert!(!assertions::findings(&workspace_results).is_empty());
    assertions::assert_rule_results(
        &workspace_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title_contains: Some("version inherited from workspace"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("`version.workspace = true`"),
            ..Default::default()
        }],
    );
}

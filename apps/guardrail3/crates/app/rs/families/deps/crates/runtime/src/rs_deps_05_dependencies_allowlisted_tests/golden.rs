use super::{
    ExpectedRuleResult, assert_rule_results, collected_facts, dependency_facts, dependency_input,
    dir_entry, project_tree,
};
use crate::facts::DependencySectionKind;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_allowlisted_runtime_dependency() {
    let facts = dependency_facts(DependencySectionKind::Dependencies, true, true, "serde");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::Dependencies,
        "serde",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            title: Some("dependency allowlisted"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn renamed_dependency_uses_package_name_for_allowlist() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"

                    [dependencies]
                    serde_alias = { package = "serde", version = "1" }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(
        &facts,
        "apps/api/Cargo.toml",
        DependencySectionKind::Dependencies,
        "serde",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            inventory: Some(true),
            message_contains: Some("`serde`"),
            ..ExpectedRuleResult::default()
        }],
    );
}

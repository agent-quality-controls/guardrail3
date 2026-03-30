use super::{collected_facts, dependency_facts, dependency_input, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_07_dev_dependencies_allowlisted as assertions;

#[test]
fn workspace_true_external_dev_dependency_keeps_warn_severity() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]

                    [workspace.dependencies]
                    tempfile = "3"
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"

                    [dev-dependencies]
                    tempfile = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "tempfile");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            message: Some(
                "Dependency `tempfile` in `[dev-dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn dev_rule_stays_silent_without_allowlist() {
    let facts = dependency_facts(false, false, "tempfile");
    let input = dependency_input(&facts, "crates/api/Cargo.toml", "tempfile");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_quiet(&results);
}

use guardrail3_domain_report::Severity;
use guardrail3_app_rs_family_deps_assertions::rs_deps_05_dependencies_allowlisted as assertions;

use super::{collected_facts, dependency_input, dir_entry, project_tree};

#[test]
fn workspace_true_external_path_dependency_is_still_checked() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages", "vendor"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
            ("vendor", dir_entry(&["reqwest"], &[])),
            ("vendor/reqwest", dir_entry(&[], &[])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]

                    [workspace.dependencies]
                    vendored_reqwest = { package = "reqwest", path = "vendor/reqwest" }
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

                    [dependencies]
                    vendored_reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(
        &facts,
        "packages/core/Cargo.toml",
        "reqwest",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(Severity::Error),
            message: Some(
                "Dependency `reqwest` in `[dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

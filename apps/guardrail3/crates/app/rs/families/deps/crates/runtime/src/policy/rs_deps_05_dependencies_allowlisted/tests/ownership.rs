use super::{collected_facts, dependency_input, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_05_dependencies_allowlisted as assertions;

#[test]
fn broad_dependency_attack_assigns_each_section_to_its_own_rule() {
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
                    [workspace]
                    members = []

                    [package]
                    name = "api"

                    [dependencies]
                    tokio = "1"

                    [build-dependencies]
                    bindgen = "0.70"

                    [dev-dependencies]
                    tempfile = "3"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    assertions::assert_broad_dependency_attack_summary(&results);
}

#[test]
fn non_workspace_path_dependency_is_still_checked() {
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
                    vendored_reqwest = { package = "reqwest", path = "../../vendor/reqwest" }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "reqwest");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Dependency `reqwest` in `[dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn workspace_path_dependency_is_skipped() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core", "support"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
            ("packages/support", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]
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
                    support = { path = "../support" }
                "#,
            ),
            (
                "packages/support/Cargo.toml",
                r#"
                    [package]
                    name = "support"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_quiet(&results);
}

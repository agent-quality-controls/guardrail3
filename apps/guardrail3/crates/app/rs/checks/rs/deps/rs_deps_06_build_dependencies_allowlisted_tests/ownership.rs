use crate::app::rs::checks::rs::deps::facts::DependencySectionKind;
use crate::app::rs::checks::rs::deps::test_support::{
    collected_facts, dependency_input, dir_entry, project_tree,
};
use crate::domain::report::Severity;

#[test]
fn workspace_true_external_build_dependency_is_checked() {
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
                    bindgen = "0.70"
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

                    [build-dependencies]
                    bindgen = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(
        &facts,
        "packages/core/Cargo.toml",
        DependencySectionKind::BuildDependencies,
        "bindgen",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-06");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(
        result.message,
        "Dependency `bindgen` in `[build-dependencies]` is not allowlisted for crate `core`."
    );
}

#[test]
fn build_rule_stays_silent_without_allowlist() {
    let facts = crate::app::rs::checks::rs::deps::test_support::dependency_facts(
        DependencySectionKind::BuildDependencies,
        false,
        false,
        "bindgen",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::BuildDependencies,
        "bindgen",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert!(results.is_empty());
}

use crate::domain::report::Severity;

use super::super::facts::DependencySectionKind;
use super::super::test_support::{
    collected_facts, dependency_facts, dependency_input, dir_entry, has_result, project_tree,
};
use super::check;

#[test]
fn inventories_allowlisted_build_dependency() {
    let facts = dependency_facts(DependencySectionKind::BuildDependencies, true, true, "cc");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::BuildDependencies,
        "cc",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn errors_on_unauthorized_build_dependency() {
    let facts = dependency_facts(
        DependencySectionKind::BuildDependencies,
        true,
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

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Dependency `bindgen` in `[build-dependencies]` is not allowlisted for crate `api`."
    );
}

#[test]
fn no_allowlist_means_build_dependency_rule_stays_silent() {
    let facts = dependency_facts(
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

    check(&input, &mut results);

    assert!(results.is_empty());
}

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

    check(&input, &mut results);

    assert!(has_result(&results, "RS-DEPS-06", |result| {
        result.severity == Severity::Error
            && result.message.contains("`bindgen`")
            && result.file.as_deref() == Some("packages/core/Cargo.toml")
    }));
}

use crate::domain::report::Severity;

use super::super::facts::DependencySectionKind;
use super::super::test_support::{
    collected_facts, dependency_facts, dependency_input, dir_entry, has_result, project_tree,
};
use super::check;

#[test]
fn inventories_allowlisted_dev_dependency() {
    let facts = dependency_facts(DependencySectionKind::DevDependencies, true, true, "insta");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "insta",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_on_unauthorized_dev_dependency() {
    let facts = dependency_facts(
        DependencySectionKind::DevDependencies,
        true,
        false,
        "tempfile",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "tempfile",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(
        results[0].message,
        "Dependency `tempfile` in `[dev-dependencies]` is not allowlisted for crate `api`."
    );
}

#[test]
fn no_allowlist_means_dev_dependency_rule_stays_silent() {
    let facts = dependency_facts(
        DependencySectionKind::DevDependencies,
        false,
        false,
        "tempfile",
    );
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "tempfile",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn workspace_true_external_dev_dependency_is_checked() {
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
    let input = dependency_input(
        &facts,
        "packages/core/Cargo.toml",
        DependencySectionKind::DevDependencies,
        "tempfile",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(has_result(&results, "RS-DEPS-07", |result| {
        result.severity == Severity::Warn
            && result.message.contains("`tempfile`")
            && result.file.as_deref() == Some("packages/core/Cargo.toml")
    }));
}

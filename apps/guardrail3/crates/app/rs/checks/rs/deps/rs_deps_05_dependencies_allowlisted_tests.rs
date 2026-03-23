use crate::domain::report::Severity;

use super::super::facts::DependencySectionKind;
use super::super::test_support::{
    collected_facts, dependency_facts, dependency_input, dir_entry, has_result, project_tree,
};
use super::check;

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

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn errors_on_unauthorized_runtime_dependency() {
    let facts = dependency_facts(DependencySectionKind::Dependencies, true, false, "tokio");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::Dependencies,
        "tokio",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].message,
        "Dependency `tokio` in `[dependencies]` is not allowlisted for crate `api`."
    );
}

#[test]
fn workspace_true_external_dependency_is_still_checked() {
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
                    reqwest = "0.12"
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
                    reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(
        &facts,
        "packages/core/Cargo.toml",
        DependencySectionKind::Dependencies,
        "reqwest",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(has_result(&results, "RS-DEPS-05", |result| {
        result.severity == Severity::Error
            && result.message.contains("`reqwest`")
            && result.file.as_deref() == Some("packages/core/Cargo.toml")
    }));
}

#[test]
fn no_allowlist_means_runtime_rule_stays_silent() {
    let facts = dependency_facts(DependencySectionKind::Dependencies, false, false, "tokio");
    let input = dependency_input(
        &facts,
        "crates/api/Cargo.toml",
        DependencySectionKind::Dependencies,
        "tokio",
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn hybrid_workspace_package_root_is_scanned() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "library"
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [package]
                    name = "rootlib"

                    [workspace]
                    members = []

                    [dependencies]
                    serde = "1"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);

    assert!(
        facts
            .dependency_entries
            .iter()
            .any(|entry| entry.cargo_rel_path == "Cargo.toml" && entry.dep_package_name == "serde")
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

    check(&input, &mut results);

    assert!(has_result(&results, "RS-DEPS-05", |result| {
        result.inventory && result.message.contains("`serde`")
    }));
}

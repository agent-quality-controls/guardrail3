use crate::app::rs::checks::rs::deps::facts::DependencySectionKind;
use crate::app::rs::checks::rs::deps::run_with_facts;
use crate::app::rs::checks::rs::deps::test_support::{
    collected_facts, dependency_input, dir_entry, project_tree,
};
use crate::domain::report::Severity;

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
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| {
            matches!(
                result.id.as_str(),
                "RS-DEPS-05" | "RS-DEPS-06" | "RS-DEPS-07"
            )
        })
        .map(|result| (result.id.as_str(), result.severity, result.message.clone()))
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            (
                "RS-DEPS-05",
                Severity::Error,
                "Dependency `tokio` in `[dependencies]` is not allowlisted for crate `api`."
                    .to_owned(),
            ),
            (
                "RS-DEPS-06",
                Severity::Error,
                "Dependency `bindgen` in `[build-dependencies]` is not allowlisted for crate `api`."
                    .to_owned(),
            ),
            (
                "RS-DEPS-07",
                Severity::Warn,
                "Dependency `tempfile` in `[dev-dependencies]` is not allowlisted for crate `api`."
                    .to_owned(),
            ),
        ]
    );
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
    let input = dependency_input(
        &facts,
        "packages/core/Cargo.toml",
        DependencySectionKind::Dependencies,
        "reqwest",
    );
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-05");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(
        result.message,
        "Dependency `reqwest` in `[dependencies]` is not allowlisted for crate `core`."
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
    let results = run_with_facts(&facts);

    assert!(results.iter().all(|result| {
        !(result.id == "RS-DEPS-05" && result.file.as_deref() == Some("packages/core/Cargo.toml"))
    }));
}

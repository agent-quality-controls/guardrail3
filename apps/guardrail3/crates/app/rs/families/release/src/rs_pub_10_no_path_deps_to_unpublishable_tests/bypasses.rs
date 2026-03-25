use super::super::super::check as run_family;
use super::super::super::release_support::dependency_edges;
use super::super::super::test_support::{
    StubToolChecker, dir_entry, edge_facts, edge_input, project_tree, temp_root,
};
use super::super::check;

#[test]
fn does_not_error_on_path_dep_to_publishable_crate() {
    let facts = edge_facts();
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn inherited_workspace_dependency_without_path_stays_out_of_rule_scope() {
    let parsed: toml::Value = toml::from_str(
        r#"
[package]
name = "example"

[dependencies]
public = { workspace = true }
"#,
    )
    .expect("valid crate manifest");
    let workspace_manifest: toml::Value = toml::from_str(
        r#"
[workspace.dependencies]
public = "1.2.3"
"#,
    )
    .expect("valid workspace manifest");
    let workspace_dependencies = workspace_manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .cloned()
        .expect("workspace dependencies");
    let edge = dependency_edges(&parsed, &workspace_dependencies)
        .into_iter()
        .find(|edge| edge.dep_name == "public")
        .expect("public edge");

    assert!(!edge.has_path);
}

#[test]
fn direct_non_path_edge_stays_out_of_rule_scope() {
    let mut facts = edge_facts();
    facts.has_path = false;
    facts.dep_publishable = false;
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn respects_actual_package_name_for_renamed_path_dependencies() {
    let root = temp_root("release-renamed-path-deps");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            (
                "crates",
                dir_entry(&["consumer", "public", "internal"], &[]),
            ),
            ("crates/consumer", dir_entry(&[], &["Cargo.toml"])),
            ("crates/public", dir_entry(&[], &["Cargo.toml"])),
            ("crates/internal", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/consumer", "crates/public", "crates/internal"]
resolver = "2"
"#,
            ),
            (
                "crates/consumer/Cargo.toml",
                r#"
[package]
name = "consumer"
version = "0.1.0"
edition = "2024"
description = "consumer"
license = "MIT"
repository = "https://example.com/consumer"

[dependencies]
public_alias = { package = "public", path = "../public", version = "1.2.3" }
internal_alias = { package = "internal", path = "../internal", version = "0.1.0" }
"#,
            ),
            (
                "crates/public/Cargo.toml",
                r#"
[package]
name = "public"
version = "1.2.3"
edition = "2024"
description = "public"
license = "MIT"
repository = "https://example.com/public"
"#,
            ),
            (
                "crates/internal/Cargo.toml",
                r#"
[package]
name = "internal"
version = "0.1.0"
edition = "2024"
publish = false
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| {
            !(result.id == "RS-PUB-10" && result.message.contains("public_alias"))
        })
    );
    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-10"
            && result.message.contains("internal_alias")
            && result.message.contains("package `internal`")
            && result.message.contains("`[dependencies]`")
            && result.file.as_deref() == Some("crates/consumer/Cargo.toml")
    }));
}

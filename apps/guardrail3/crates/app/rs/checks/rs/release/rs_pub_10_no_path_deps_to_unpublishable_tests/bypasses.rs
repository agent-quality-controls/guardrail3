use super::super::super::release_support::dependency_edges;
use super::super::super::test_support::{edge_facts, edge_input};
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

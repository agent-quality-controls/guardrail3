use guardrail3_domain_report::Severity;

use super::super::{check, dependency_edges, edge_facts, edge_input};

#[test]
fn errors_on_inherited_workspace_path_dep_to_unpublishable_crate() {
    let parsed: toml::Value = toml::from_str(
        r#"
[package]
name = "example"

[dependencies]
internal = { workspace = true }
"#,
    )
    .expect("valid crate manifest");
    let workspace_manifest: toml::Value = toml::from_str(
        r#"
[workspace.dependencies]
internal = { path = "../internal", version = "1.2.3" }
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
        .find(|edge| edge.dep_name == "internal")
        .expect("internal edge");
    assert!(edge.has_path);

    let mut facts = edge_facts();
    facts.crate_name = "example".to_owned();
    facts.cargo_rel_path = "crates/example/Cargo.toml".to_owned();
    facts.dep_name = edge.dep_name;
    facts.dep_package_name = edge.dep_package_name;
    facts.section_label = edge.section_label;
    facts.target_label = edge.target_label;
    facts.has_path = edge.has_path;
    facts.dep_publishable = false;
    facts.version_req = edge.version_req;
    facts.actual_version = Some("1.2.3".to_owned());
    facts.version_satisfied = Some(true);
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-10");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].message.contains("Dependency `internal`"));
    assert!(results[0].message.contains("`[dependencies]`"));
}

#[test]
fn errors_on_target_specific_path_dep_to_unpublishable_crate() {
    let parsed: toml::Value = toml::from_str(
        r#"
[package]
name = "example"

[target.'cfg(unix)'.build-dependencies]
internal = { path = "../internal", version = "1.2.3" }
"#,
    )
    .expect("valid crate manifest");
    let edge = dependency_edges(&parsed, &toml::map::Map::new())
        .into_iter()
        .find(|edge| edge.dep_name == "internal")
        .expect("internal edge");
    assert!(edge.has_path);
    assert_eq!(edge.section_label, "build-dependencies");
    assert_eq!(edge.target_label.as_deref(), Some("cfg(unix)"));

    let mut facts = edge_facts();
    facts.crate_name = "example".to_owned();
    facts.cargo_rel_path = "crates/example/Cargo.toml".to_owned();
    facts.dep_name = edge.dep_name;
    facts.dep_package_name = edge.dep_package_name;
    facts.section_label = edge.section_label;
    facts.target_label = edge.target_label;
    facts.has_path = edge.has_path;
    facts.dep_publishable = false;
    facts.version_req = edge.version_req;
    facts.actual_version = Some("1.2.3".to_owned());
    facts.version_satisfied = Some(true);
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-10");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].message.contains("`[build-dependencies]`"));
    assert!(results[0].message.contains("under target `cfg(unix)`"));
}

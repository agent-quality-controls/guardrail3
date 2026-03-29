use guardrail3_domain_report::Severity;

use super::super::{check, dependency_edges, edge_facts, edge_input};

#[test]
fn errors_on_inherited_workspace_path_dep_with_incompatible_version() {
    let parsed: toml::Value = toml::from_str(
        r#"
[package]
name = "example"

[dependencies]
api = { workspace = true }
"#,
    )
    .expect("valid crate manifest");
    let workspace_manifest: toml::Value = toml::from_str(
        r#"
[workspace.dependencies]
api = { path = "../api", version = "^2.0.0" }
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
        .find(|edge| edge.dep_name == "api")
        .expect("api edge");
    assert!(edge.has_path);
    assert_eq!(edge.version_req.as_deref(), Some("^2.0.0"));

    let mut facts = edge_facts();
    facts.crate_name = "example".to_owned();
    facts.cargo_rel_path = "crates/example/Cargo.toml".to_owned();
    facts.dep_name = edge.dep_name;
    facts.dep_package_name = edge.dep_package_name;
    facts.section_label = edge.section_label;
    facts.target_label = edge.target_label;
    facts.has_path = edge.has_path;
    facts.dep_publishable = true;
    facts.version_req = edge.version_req;
    facts.actual_version = Some("1.2.3".to_owned());
    facts.version_satisfied = Some(false);
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-11");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("version mismatch with api"));
    assert!(results[0].message.contains("Dependency `api`"));
    assert!(results[0].message.contains("`[dependencies]`"));
    assert!(results[0].message.contains("requires `^2.0.0`"));
    assert!(
        results[0]
            .message
            .contains("actual local publishable version is `1.2.3`")
    );
}

#[test]
fn errors_on_target_specific_renamed_path_dep_with_incompatible_version() {
    let mut facts = edge_facts();
    facts.crate_name = "example".to_owned();
    facts.cargo_rel_path = "crates/example/Cargo.toml".to_owned();
    facts.dep_name = "api_v2".to_owned();
    facts.dep_package_name = "api".to_owned();
    facts.section_label = "build-dependencies".to_owned();
    facts.target_label = Some("cfg(unix)".to_owned());
    facts.has_path = true;
    facts.dep_publishable = true;
    facts.version_req = Some("^2.0.0".to_owned());
    facts.actual_version = Some("1.2.3".to_owned());
    facts.version_satisfied = Some(false);
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-11");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].message.contains("Dependency `api_v2`"));
    assert!(results[0].message.contains("package `api`"));
    assert!(results[0].message.contains("`[build-dependencies]`"));
    assert!(results[0].message.contains("under target `cfg(unix)`"));
    assert!(results[0].message.contains("requires `^2.0.0`"));
    assert!(
        results[0]
            .message
            .contains("actual local publishable version is `1.2.3`")
    );
}

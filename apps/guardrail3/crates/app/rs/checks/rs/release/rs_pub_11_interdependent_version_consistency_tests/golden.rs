use crate::domain::report::Severity;

use super::super::super::facts::ReleaseEdgeFacts;
use super::super::super::inputs::ReleaseEdgeInput;
use super::super::super::release_support::dependency_edges;
use super::super::check;

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

    let facts = ReleaseEdgeFacts {
        crate_name: "example".to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        dep_name: edge.dep_name,
        dep_package_name: edge.dep_package_name,
        section_label: edge.section_label,
        target_label: edge.target_label,
        has_path: edge.has_path,
        dep_publishable: true,
        version_req: edge.version_req,
        actual_version: Some("1.2.3".to_owned()),
        version_satisfied: Some(false),
    };
    let input = ReleaseEdgeInput::new(&facts);
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
    let facts = ReleaseEdgeFacts {
        crate_name: "example".to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        dep_name: "api_v2".to_owned(),
        dep_package_name: "api".to_owned(),
        section_label: "build-dependencies".to_owned(),
        target_label: Some("cfg(unix)".to_owned()),
        has_path: true,
        dep_publishable: true,
        version_req: Some("^2.0.0".to_owned()),
        actual_version: Some("1.2.3".to_owned()),
        version_satisfied: Some(false),
    };
    let input = ReleaseEdgeInput::new(&facts);
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

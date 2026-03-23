use crate::domain::report::Severity;

use super::super::test_support::{
    collected_facts, dir_entry, failure_facts, failure_input, project_tree,
};
use super::check;

#[test]
fn emits_error_for_input_failure() {
    let facts = failure_facts("guardrail3.toml", "parse failed");
    let input = failure_input(&facts, "guardrail3.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-DEPS-11");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].message, "parse failed");
}

#[test]
fn collect_surfaces_guardrail_parse_failure() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["guardrail3.toml"]))],
        vec![("guardrail3.toml", "[rust.apps")],
    );
    let facts = collected_facts(&tree, &[]);
    let input = failure_input(&facts, "guardrail3.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].file.as_deref(), Some("guardrail3.toml"));
    assert!(
        results[0]
            .message
            .contains("Failed to parse guardrail3.toml")
    );
}

#[test]
fn collect_surfaces_unresolved_workspace_dependency() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["crates"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("crates", dir_entry(&["api"], &[])),
            ("crates/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]"),
            ("guardrail3.toml", "[rust.apps.api]\nprofile = \"service\""),
            (
                "crates/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"

                    [dependencies]
                    reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = failure_input(&facts, "crates/api/Cargo.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert!(
        results[0]
            .message
            .contains("[workspace.dependencies].reqwest")
    );
}

#[test]
fn collect_surfaces_workspace_member_without_cargo_toml() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["api"], &[])),
            ("crates/api", dir_entry(&[], &[])),
        ],
        vec![("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]")],
    );
    let facts = collected_facts(&tree, &[]);
    let input = failure_input(&facts, "crates/api");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert!(results[0].message.contains("has no Cargo.toml"));
}

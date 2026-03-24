use crate::app::rs::checks::rs::deps::run_with_facts;
use crate::app::rs::checks::rs::deps::test_support::{collected_facts, dir_entry, project_tree};
use crate::domain::report::Severity;

#[test]
fn collect_surfaces_guardrail_parse_failure() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["guardrail3.toml"]))],
        vec![("guardrail3.toml", "[rust.apps")],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-11")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.message.contains("Failed to parse guardrail3.toml"),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![(Some("guardrail3.toml"), Severity::Error, true)]
    );
}

#[test]
fn malformed_member_manifest_surfaces_explicit_failure() {
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
                "#,
            ),
            ("apps/api/Cargo.toml", "[[broken"),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-11")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result
                    .message
                    .contains("Failed to parse Cargo.toml for dependency policy check"),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![(Some("apps/api/Cargo.toml"), Severity::Error, true)]
    );
}

#[test]
fn malformed_workspace_manifest_does_not_fail_open_workspace_true_resolution() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("Cargo.toml", "[[broken"),
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
                    reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-11")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result
                    .message
                    .contains("Failed to parse workspace Cargo.toml")
                    || result.message.contains("workspace = true"),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            (Some("Cargo.toml"), Severity::Error, true),
            (Some("apps/api/Cargo.toml"), Severity::Error, true),
        ]
    );
}

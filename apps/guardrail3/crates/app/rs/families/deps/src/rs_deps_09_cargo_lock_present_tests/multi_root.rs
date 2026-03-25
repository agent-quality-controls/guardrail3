use crate::run_with_facts;
use crate::test_support::{collected_facts, dir_entry, project_tree};
use guardrail3_domain_report::Severity;

#[test]
fn missing_lockfiles_across_multiple_roots_keep_exact_severities() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps", "packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "service"

                    [rust.apps.api]
                    profile = "service"

                    [rust.packages]
                    profile = "library"
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-09")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.message.as_str(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            (
                Some("Cargo.lock"),
                Severity::Error,
                "Non-library Rust root `.` is missing `Cargo.lock`.",
                false,
            ),
            (
                Some("apps/api/Cargo.lock"),
                Severity::Error,
                "Non-library Rust root `apps/api` is missing `apps/api/Cargo.lock`.",
                false,
            ),
            (
                Some("packages/core/Cargo.lock"),
                Severity::Info,
                "Library-profile Rust root `packages/core` is missing `packages/core/Cargo.lock`.",
                false,
            ),
        ]
    );
}

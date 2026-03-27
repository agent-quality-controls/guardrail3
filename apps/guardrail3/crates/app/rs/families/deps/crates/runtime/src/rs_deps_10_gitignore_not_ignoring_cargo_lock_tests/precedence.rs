use super::{collected_facts, dir_entry, project_tree};
use crate::run_with_facts;
use guardrail3_domain_report::Severity;

#[test]
fn reports_exact_gitignore_sources_across_roots() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["apps", "packages"],
                    &[".gitignore", "Cargo.toml", "Cargo.lock", "guardrail3.toml"],
                ),
            ),
            ("apps", dir_entry(&["api"], &[])),
            (
                "apps/api",
                dir_entry(&[], &[".gitignore", "Cargo.toml", "Cargo.lock"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            (
                "packages/core",
                dir_entry(&[], &["Cargo.toml", "Cargo.lock"]),
            ),
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
            (".gitignore", "packages/core/Cargo.lock"),
            ("apps/api/.gitignore", "Cargo.lock"),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-10")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.inventory,
                result.message.as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            (
                Some("Cargo.lock"),
                Severity::Info,
                true,
                "No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`.",
            ),
            (
                Some("apps/api/.gitignore"),
                Severity::Error,
                false,
                "`apps/api/.gitignore` ignores `apps/api/Cargo.lock` for Rust root `apps/api`.",
            ),
            (
                Some(".gitignore"),
                Severity::Error,
                false,
                "`.gitignore` ignores `packages/core/Cargo.lock` for Rust root `packages/core`.",
            ),
        ]
    );
}

#[test]
fn nested_unignore_overrides_ancestor_ignore() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[".gitignore", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            (
                "apps/api",
                dir_entry(&[], &[".gitignore", "Cargo.toml", "Cargo.lock"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
            (".gitignore", "**/Cargo.lock"),
            ("apps/api/.gitignore", "!Cargo.lock"),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-10")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.inventory,
                result.message.as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![(
            Some("apps/api/Cargo.lock"),
            Severity::Info,
            true,
            "No relevant `.gitignore` masks `apps/api/Cargo.lock` for Rust root `apps/api`.",
        )]
    );
}

#[test]
fn anchored_root_cargo_lock_pattern_does_not_collapse_to_nested_roots() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["apps"],
                    &[".gitignore", "Cargo.toml", "Cargo.lock", "guardrail3.toml"],
                ),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml", "Cargo.lock"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "service"

                    [rust.apps.api]
                    profile = "service"
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
            (".gitignore", "/Cargo.lock"),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-10")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.inventory,
                result.message.as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            (
                Some(".gitignore"),
                Severity::Error,
                false,
                "`.gitignore` ignores `Cargo.lock` for Rust root `.`.",
            ),
            (
                Some("apps/api/Cargo.lock"),
                Severity::Info,
                true,
                "No relevant `.gitignore` masks `apps/api/Cargo.lock` for Rust root `apps/api`.",
            ),
        ]
    );
}

#[test]
fn anchored_root_cargo_glob_pattern_stays_anchored() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["apps"],
                    &[".gitignore", "Cargo.toml", "Cargo.lock", "guardrail3.toml"],
                ),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml", "Cargo.lock"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "service"

                    [rust.apps.api]
                    profile = "service"
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
            (".gitignore", "/Cargo.*"),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id == "RS-DEPS-10")
        .map(|result| {
            (
                result.file.as_deref(),
                result.severity,
                result.inventory,
                result.message.as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            (
                Some(".gitignore"),
                Severity::Error,
                false,
                "`.gitignore` ignores `Cargo.lock` for Rust root `.`.",
            ),
            (
                Some("apps/api/Cargo.lock"),
                Severity::Info,
                true,
                "No relevant `.gitignore` masks `apps/api/Cargo.lock` for Rust root `apps/api`.",
            ),
        ]
    );
}

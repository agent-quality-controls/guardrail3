use super::{collected_facts, collected_facts_with_validation_scope, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_10_gitignore_not_ignoring_cargo_lock as assertions;

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
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-10")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.inventory(),
                result.message(),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![
            (
                Some("Cargo.lock"),
                assertions::Severity::Info,
                true,
                "No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`.",
            ),
            (
                Some("apps/api/.gitignore"),
                assertions::Severity::Error,
                false,
                "`apps/api/.gitignore` ignores `apps/api/Cargo.lock` for Rust root `apps/api`.",
            ),
        ],
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
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-10")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.inventory(),
                result.message(),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(
            Some("apps/api/Cargo.lock"),
            assertions::Severity::Info,
            true,
            "No relevant `.gitignore` masks `apps/api/Cargo.lock` for Rust root `apps/api`.",
        )],
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
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-10")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.inventory(),
                result.message(),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![
            (
                Some(".gitignore"),
                assertions::Severity::Error,
                false,
                "`.gitignore` ignores `Cargo.lock` for Rust root `.`.",
            ),
            (
                Some("apps/api/Cargo.lock"),
                assertions::Severity::Info,
                true,
                "No relevant `.gitignore` masks `apps/api/Cargo.lock` for Rust root `apps/api`.",
            ),
        ],
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
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-10")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.inventory(),
                result.message(),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![
            (
                Some(".gitignore"),
                assertions::Severity::Error,
                false,
                "`.gitignore` ignores `Cargo.lock` for Rust root `.`.",
            ),
            (
                Some("apps/api/Cargo.lock"),
                assertions::Severity::Info,
                true,
                "No relevant `.gitignore` masks `apps/api/Cargo.lock` for Rust root `apps/api`.",
            ),
        ],
    );
}

#[test]
fn scoped_run_ignores_unrelated_standalone_package_gitignore_results() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["apps", "tools"],
                    &[".gitignore", "Cargo.toml", "Cargo.lock", "guardrail3.toml"],
                ),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("tools", dir_entry(&["helper"], &[])),
            (
                "tools/helper",
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
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/*"]
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                "#,
            ),
            (
                "tools/helper/Cargo.toml",
                r#"
                    [package]
                    name = "helper"
                "#,
            ),
            (".gitignore", "tools/helper/Cargo.lock"),
        ],
    );

    let facts = collected_facts_with_validation_scope(
        &tree,
        &["cargo-deny", "cargo-machete", "cargo-dupes", "gitleaks"],
        Some("apps/api"),
    );
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-10")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.inventory(),
                result.message(),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(
            Some("Cargo.lock"),
            assertions::Severity::Info,
            true,
            "No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`.",
        )],
    );
}

#[test]
fn nested_non_member_helper_crate_under_workspace_root_is_not_a_gitignore_root() {
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
            ("apps/api", dir_entry(&["assertions"], &["Cargo.toml"])),
            (
                "apps/api/assertions",
                dir_entry(&[], &["Cargo.toml", "Cargo.lock"]),
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
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/api"]
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                "#,
            ),
            (
                "apps/api/assertions/Cargo.toml",
                r#"
                    [package]
                    name = "api-assertions"
                "#,
            ),
            (".gitignore", "apps/api/assertions/Cargo.lock"),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    let summary = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-10")
        .map(|result| {
            (
                result.file(),
                result.severity(),
                result.inventory(),
                result.message(),
            )
        })
        .collect::<Vec<_>>();

    assertions::assert_summary(
        summary,
        vec![(
            Some("Cargo.lock"),
            assertions::Severity::Info,
            true,
            "No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`.",
        )],
    );
}

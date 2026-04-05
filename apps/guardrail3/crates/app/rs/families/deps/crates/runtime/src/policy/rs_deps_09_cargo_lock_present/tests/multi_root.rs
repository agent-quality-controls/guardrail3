use super::{collected_facts, collected_facts_with_validation_scope, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_09_cargo_lock_present as assertions;
use guardrail3_app_rs_family_deps_assertions::rs_deps_11_input_failures as input_failure_assertions;

#[test]
fn missing_lockfiles_across_multiple_roots_keep_exact_severities() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps", "packages"], &["guardrail3.toml"])),
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
                "apps/api/Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/api/Cargo.lock"),
                severity: Some(assertions::Severity::Error),
                message: Some("`apps/api` is missing `apps/api/Cargo.lock`. Run `cargo generate-lockfile` and commit the result."),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                file: Some("packages/core/Cargo.lock"),
                severity: Some(assertions::Severity::Info),
                message: Some(
                    "Library-profile Rust root `packages/core` is missing `packages/core/Cargo.lock`.",
                ),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}

#[test]
fn nested_package_root_uses_library_lockfile_severity() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["tools"], &["guardrail3.toml"])),
            ("tools", dir_entry(&["packages"], &[])),
            ("tools/packages", dir_entry(&["core"], &[])),
            ("tools/packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "service"

                    [rust.packages]
                    profile = "library"
                "#,
            ),
            (
                "tools/packages/core/Cargo.toml",
                r#"
                    [workspace]
                    members = []
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("tools/packages/core/Cargo.lock"),
            severity: Some(assertions::Severity::Info),
            message: Some(
                "Library-profile Rust root `tools/packages/core` is missing `tools/packages/core/Cargo.lock`.",
            ),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn nested_standalone_package_under_workspace_root_is_not_a_lockfile_root() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["apps", "tools"],
                    &["Cargo.toml", "Cargo.lock", "guardrail3.toml"],
                ),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("tools", dir_entry(&["helper"], &[])),
            ("tools/helper", dir_entry(&[], &["Cargo.toml"])),
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
        ],
    );

    let facts = collected_facts(
        &tree,
        &["cargo-deny", "cargo-machete", "cargo-dupes", "gitleaks"],
    );
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("Cargo.lock"),
            severity: Some(assertions::Severity::Info),
            message: Some("Rust root `.` has `Cargo.lock` committed."),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    assert!(
        results
            .iter()
            .all(|result| result.file() != Some("tools/helper/Cargo.lock")),
        "nested package under workspace root incorrectly became a lockfile root: {results:#?}"
    );
}

#[test]
fn scoped_run_ignores_unrelated_standalone_package_roots() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["apps", "tools"],
                    &["Cargo.toml", "Cargo.lock", "guardrail3.toml"],
                ),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("tools", dir_entry(&["helper"], &[])),
            ("tools/helper", dir_entry(&[], &["Cargo.toml"])),
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
        ],
    );

    let facts = collected_facts_with_validation_scope(
        &tree,
        &["cargo-deny", "cargo-machete", "cargo-dupes", "gitleaks"],
        Some("apps/api"),
    );
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("Cargo.lock"),
            severity: Some(assertions::Severity::Info),
            message: Some("Rust root `.` has `Cargo.lock` committed."),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn nested_non_member_package_under_workspace_root_emits_input_failure() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["support"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("support", dir_entry(&["assertions"], &[])),
            ("support/assertions", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [profile]
                    name = "service"
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
                "support/assertions/Cargo.toml",
                r#"
                    [package]
                    name = "support-assertions"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("Cargo.lock"),
            severity: Some(assertions::Severity::Error),
            message: Some("`.` is missing `Cargo.lock`. Run `cargo generate-lockfile` and commit the result."),
            inventory: Some(false),
            ..Default::default()
        }],
    );
    input_failure_assertions::assert_rule_results(&results, &[]);
    assert!(
        results
            .iter()
            .all(|result| result.file() != Some("support/assertions/Cargo.lock")),
        "nested non-member package incorrectly became a lockfile root: {results:#?}"
    );
}

#[test]
fn nested_non_member_helper_crate_under_workspace_root_emits_input_failure_without_lockfile_root() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "Cargo.lock"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["assertions"], &["Cargo.toml"])),
            ("apps/api/assertions", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
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
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("Cargo.lock"),
            severity: Some(assertions::Severity::Info),
            message: Some("Rust root `.` has `Cargo.lock` committed."),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    input_failure_assertions::assert_rule_results(&results, &[]);
    assert!(
        results
            .iter()
            .all(|result| result.file() != Some("apps/api/assertions/Cargo.lock")),
        "nested helper crate incorrectly became a lockfile root: {results:#?}"
    );
}

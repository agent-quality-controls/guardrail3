use super::{collected_facts, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_09_cargo_lock_present as assertions;
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
    let results = super::run_with_facts(&facts);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("Cargo.lock"),
                severity: Some(Severity::Error),
                message: Some("Non-library Rust root `.` is missing `Cargo.lock`."),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/api/Cargo.lock"),
                severity: Some(Severity::Error),
                message: Some("Non-library Rust root `apps/api` is missing `apps/api/Cargo.lock`."),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                file: Some("packages/core/Cargo.lock"),
                severity: Some(Severity::Info),
                message: Some("Library-profile Rust root `packages/core` is missing `packages/core/Cargo.lock`."),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}

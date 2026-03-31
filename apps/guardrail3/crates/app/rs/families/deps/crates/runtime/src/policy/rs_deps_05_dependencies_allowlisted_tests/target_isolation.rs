use super::{collected_facts, collected_facts_with_validation_scope, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_05_dependencies_allowlisted as assertions;
use guardrail3_app_rs_family_deps_assertions::rs_deps_12_direct_dependency_cap::{
    ExpectedInputFailureResult, InputFailureSeverity, assert_input_failure_results,
};

#[test]
fn malformed_target_table_does_not_suppress_top_level_allowlist_violation() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/*"]
                "#,
            ),
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
                    tokio = "1"

                    [target.'cfg(unix)'.dependencies]
                    broken = 123
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("apps/api/Cargo.toml"),
            message: Some(
                "Dependency `tokio` in `[dependencies]` is not allowlisted for crate `api`.",
            ),
            inventory: Some(false),
            ..Default::default()
        }],
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-11"
                && result.file() == Some("apps/api/Cargo.toml")
                && result
                    .message()
                    .contains("`[target.cfg(unix).dependencies].broken` must be a string or table.")
        }),
        "expected RS-DEPS-11 target-table failure: {results:#?}"
    );
}

#[test]
fn target_specific_runtime_dependency_is_checked() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
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

                    [target.'cfg(unix)'.dependencies]
                    reqwest = "0.12"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/api/Cargo.toml"),
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Dependency `reqwest` in `[target.'cfg(unix)'.dependencies]` is not allowlisted for crate `api`.",
            ),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn scoped_run_does_not_report_sibling_allowlist_violations() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps"], &["Cargo.toml", "Cargo.lock", "guardrail3.toml"]),
            ),
            ("apps", dir_entry(&["api", "admin"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("apps/admin", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/*"]
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowed_deps = ["serde"]

                    [rust.apps.admin]
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
                    reqwest = "0.12"
                "#,
            ),
            (
                "apps/admin/Cargo.toml",
                r#"
                    [package]
                    name = "admin"

                    [dependencies]
                    tokio = "1"
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
            severity: Some(assertions::Severity::Error),
            file: Some("apps/api/Cargo.toml"),
            message: Some(
                "Dependency `reqwest` in `[dependencies]` is not allowlisted for crate `api`.",
            ),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn nested_non_member_helper_crate_under_workspace_root_emits_input_failure() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
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
                "#,
            ),
            (
                "apps/api/assertions/Cargo.toml",
                r#"
                    [package]
                    name = "api-assertions"

                    [dependencies]
                    reqwest = "0.12"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_quiet(&results);
    assert_input_failure_results(
        &results,
        &[ExpectedInputFailureResult {
            severity: Some(InputFailureSeverity::Error),
            file: Some("apps/api/assertions/Cargo.toml"),
            message_contains: Some("not declared as a workspace package"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

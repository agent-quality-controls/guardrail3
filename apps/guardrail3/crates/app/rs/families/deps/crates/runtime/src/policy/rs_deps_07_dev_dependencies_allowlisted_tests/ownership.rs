use super::{collected_facts, dependency_facts, dependency_input, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_07_dev_dependencies_allowlisted as assertions;

#[test]
fn workspace_true_external_dev_dependency_keeps_warn_severity() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]

                    [workspace.dependencies]
                    tempfile = "3"
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"

                    [dev-dependencies]
                    tempfile = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "tempfile");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            message: Some(
                "Dependency `tempfile` in `[dev-dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn target_specific_dev_dependency_keeps_warn_severity() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/core"]
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "packages/core/Cargo.toml",
                r#"
                    [package]
                    name = "core"

                    [target.'cfg(unix)'.dev-dependencies]
                    tempfile = "3"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "tempfile");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            message: Some(
                "Dependency `tempfile` in `[target.'cfg(unix)'.dev-dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn dev_rule_stays_silent_without_allowlist() {
    let facts = dependency_facts(false, false, "tempfile");
    let input = dependency_input(&facts, "crates/api/Cargo.toml", "tempfile");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_quiet(&results);
}

#[test]
fn undeclared_workspace_path_dev_dependency_fails_closed() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["tools"], &["Cargo.toml", "guardrail3.toml"])),
            ("tools", dir_entry(&["packages"], &[])),
            ("tools/packages", dir_entry(&["runtime"], &[])),
            (
                "tools/packages/runtime",
                dir_entry(&["assertions"], &["Cargo.toml"]),
            ),
            (
                "tools/packages/runtime/assertions",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["tools/packages/runtime"]
                "#,
            ),
            (
                "guardrail3.toml",
                r#"
                    [rust.packages]
                    profile = "library"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "tools/packages/runtime/Cargo.toml",
                r#"
                    [package]
                    name = "runtime"

                    [dev-dependencies]
                    runtime_assertions = { path = "assertions" }
                "#,
            ),
            (
                "tools/packages/runtime/assertions/Cargo.toml",
                r#"
                    [package]
                    name = "runtime-assertions"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let results = super::run_with_facts(&facts);

    assertions::assert_rule_quiet(&results);
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-11"
                && result.file() == Some("tools/packages/runtime/Cargo.toml")
                && result.message().contains("not declared in `[workspace].members`")
        }),
        "expected undeclared local workspace package failure: {results:#?}"
    );
}

use super::{collected_facts, dependency_facts, dependency_input, dir_entry, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_06_build_dependencies_allowlisted as assertions;

#[test]
fn workspace_true_external_build_dependency_is_checked() {
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
                    bindgen = "0.70"
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

                    [build-dependencies]
                    bindgen = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "bindgen");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Dependency `bindgen` in `[build-dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn target_specific_build_dependency_is_checked() {
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

                    [target.'cfg(unix)'.build-dependencies]
                    bindgen = "0.70"
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "bindgen");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Dependency `bindgen` in `[target.'cfg(unix)'.build-dependencies]` is not allowlisted for crate `core`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn build_rule_stays_silent_without_allowlist() {
    let facts = dependency_facts(false, false, "bindgen");
    let input = dependency_input(&facts, "crates/api/Cargo.toml", "bindgen");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_quiet(&results);
}

#[test]
fn undeclared_workspace_path_build_dependency_fails_closed() {
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

                    [build-dependencies]
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

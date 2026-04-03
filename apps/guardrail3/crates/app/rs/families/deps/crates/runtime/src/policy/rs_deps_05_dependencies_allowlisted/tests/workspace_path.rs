use guardrail3_app_rs_family_deps_assertions::rs_deps_05_dependencies_allowlisted as assertions;

use super::{collected_facts, dependency_input, dir_entry, project_tree};

#[test]
fn workspace_true_external_path_dependency_is_still_checked() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages", "vendor"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("packages", dir_entry(&["core"], &[])),
            ("packages/core", dir_entry(&[], &["Cargo.toml"])),
            ("vendor", dir_entry(&["reqwest"], &[])),
            ("vendor/reqwest", dir_entry(&[], &[])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["packages/*"]

                    [workspace.dependencies]
                    vendored_reqwest = { package = "reqwest", path = "vendor/reqwest" }
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

                    [dependencies]
                    vendored_reqwest = { workspace = true }
                "#,
            ),
        ],
    );
    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "packages/core/Cargo.toml", "reqwest");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Dependency `reqwest` in `[dependencies]` is not allowlisted for crate `core`. Add it to the dependency allowlist or remove the dependency.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn undeclared_workspace_path_dependency_fails_closed() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["tools"], &["Cargo.toml", "Cargo.lock", "guardrail3.toml"]),
            ),
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

                    [dependencies]
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
                && result
                    .message()
                    .contains("not declared in `[workspace].members`")
        }),
        "expected undeclared local workspace package failure: {results:#?}"
    );
}

#[test]
fn nested_app_zone_uses_app_policy_anywhere_in_path() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["tools"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("tools", dir_entry(&["apps"], &[])),
            ("tools/apps", dir_entry(&["api"], &[])),
            ("tools/apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("tools/apps/api/crates", dir_entry(&["worker"], &[])),
            (
                "tools/apps/api/crates/worker",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["tools/apps/api", "tools/apps/api/crates/worker"]
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
                "tools/apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                "#,
            ),
            (
                "tools/apps/api/crates/worker/Cargo.toml",
                r#"
                    [package]
                    name = "worker"

                    [dependencies]
                    reqwest = "0.12"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let input = dependency_input(&facts, "tools/apps/api/crates/worker/Cargo.toml", "reqwest");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("tools/apps/api/crates/worker/Cargo.toml"),
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Dependency `reqwest` in `[dependencies]` is not allowlisted for crate `worker`. Add it to the dependency allowlist or remove the dependency.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn missing_workspace_dependency_entry_fails_closed_without_allowlist_result() {
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

                    [dependencies]
                    serde = { workspace = true }
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
                && result.file() == Some("packages/core/Cargo.toml")
                && result.message().contains("[workspace.dependencies].serde")
        }),
        "expected workspace dependency resolution failure: {results:#?}"
    );
}

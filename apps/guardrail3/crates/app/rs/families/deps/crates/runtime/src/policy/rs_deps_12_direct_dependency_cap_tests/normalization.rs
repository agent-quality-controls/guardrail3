use guardrail3_app_rs_family_deps_assertions::rs_deps_12_direct_dependency_cap as assertions;

use super::{collected_facts, dir_entry, project_tree, run_with_facts};

#[test]
fn deduplicates_package_names_across_sections_aliases_and_targets() {
    let unique_dependencies = (0..24)
        .map(|index| format!("dep{index} = \"1\""))
        .collect::<Vec<_>>()
        .join("\n");
    let manifest = format!(
        r#"
            [package]
            name = "api"

            [dependencies]
            {unique_dependencies}
            serde = "1"

            [build-dependencies]
            serde_build = {{ package = "serde", version = "1" }}

            [dev-dependencies]
            serde_dev = {{ package = "serde", version = "1" }}

            [target.'cfg(unix)'.dependencies]
            serde_unix = {{ package = "serde", version = "1" }}
        "#
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml"])),
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
            ("apps/api/Cargo.toml", &manifest),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);

    assertions::assert_rule_quiet(&results);
}

#[test]
fn counts_external_workspace_and_vendored_path_but_skips_internal_workspace_path() {
    let filler_dependencies = (0..23)
        .map(|index| format!("dep{index} = \"1\""))
        .collect::<Vec<_>>()
        .join("\n");
    let manifest = format!(
        r#"
            [package]
            name = "api"

            [dependencies]
            {filler_dependencies}
            support = {{ path = "../support" }}
            vendored_reqwest = {{ package = "reqwest", path = "../../vendor/reqwest" }}
            serde = {{ workspace = true }}

            [target.'cfg(unix)'.dependencies]
            bytes = "1"
        "#
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps", "vendor"], &["Cargo.toml"])),
            ("apps", dir_entry(&["api", "support"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
            ("apps/support", dir_entry(&[], &["Cargo.toml"])),
            ("vendor", dir_entry(&["reqwest"], &[])),
            ("vendor/reqwest", dir_entry(&[], &[])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/*"]

                    [workspace.dependencies]
                    serde = "1"
                "#,
            ),
            ("apps/api/Cargo.toml", &manifest),
            (
                "apps/support/Cargo.toml",
                r#"
                    [package]
                    name = "support"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("too many direct dependencies"),
            file: Some("apps/api/Cargo.toml"),
            message: Some("Crate `api` has 26 unique direct dependencies (max 25)."),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn malformed_target_dependency_table_fails_closed_through_input_failures() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml"])),
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
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"

                    [target.'cfg(unix)'.dependencies]
                    broken = 123
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);

    assertions::assert_rule_quiet(&results);
    assertions::assert_input_failure_results(
        &results,
        &[assertions::ExpectedInputFailureResult {
            severity: Some(assertions::InputFailureSeverity::Error),
            file: Some("apps/api/Cargo.toml"),
            message_contains: Some(
                "`[target.cfg(unix).dependencies].broken` must be a string or table.",
            ),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_app_rs_family_release_assertions::repo_inventory::rs_release_12_input_failures as assertions;
use guardrail3_app_rs_family_release_assertions::repo_inventory::rs_release_12_input_failures::ExpectedRuleResult;

#[test]
fn emits_exact_fail_closed_hits_for_malformed_release_configs_and_workflow_yaml() {
    let root = temp_root("release-input-failures-bad-configs");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &[".github"],
                    &["Cargo.toml", "release-plz.toml", "cliff.toml"],
                ),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["release.yml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
resolver = "2"

[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            ("release-plz.toml", "[workspace"),
            ("cliff.toml", "[git"),
            (".github/workflows/release.yml", "jobs:\n  release: ["),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);
    assertions::assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                file: Some(".github/workflows/release.yml"),
                ..ExpectedRuleResult::default()
            },
            ExpectedRuleResult {
                file: Some("cliff.toml"),
                ..ExpectedRuleResult::default()
            },
            ExpectedRuleResult {
                file: Some("release-plz.toml"),
                ..ExpectedRuleResult::default()
            },
        ],
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::RELEASE_PLZ_COVERAGE_RULE_ID,
        "release-plz.toml",
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::CLIFF_EXISTS_RULE_ID,
        "cliff.toml",
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::RELEASE_WORKFLOW_RULE_ID,
        ".github/workflows/release.yml",
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::PUBLISH_DRY_RUN_WORKFLOW_RULE_ID,
        ".github/workflows/release.yml",
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::REGISTRY_TOKEN_RULE_ID,
        ".github/workflows/release.yml",
    );
}

#[test]
fn cargo_parse_failure_is_reported_without_suppressing_other_repo_results() {
    let root = temp_root("release-input-failures-cargo");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github", "crates"], &["Cargo.toml", "release-plz.toml"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["release.yml"])),
            ("crates", dir_entry(&["example"], &[])),
            ("crates/example", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/example"]
resolver = "2"
"#,
            ),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
"#,
            ),
            ("crates/example/Cargo.toml", "[package"),
            (
                ".github/workflows/release.yml",
                r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz release-pr
"#,
            ),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("crates/example/Cargo.toml"),
            ..ExpectedRuleResult::default()
        }],
    );
    assertions::assert_related_rule_results(
        &results,
        assertions::RELEASE_PLZ_EXISTS_RULE_ID,
        &[ExpectedRuleResult {
            file: Some("release-plz.toml"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
    assertions::assert_related_rule_results(
        &results,
        assertions::RELEASE_WORKFLOW_RULE_ID,
        &[ExpectedRuleResult {
            file: Some(".github/workflows/release.yml"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn unreadable_cached_release_files_fail_closed_when_structure_exists_without_content() {
    let root = temp_root("release-input-failures-unreadable-cached-files");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &[".github", "crates"],
                    &["Cargo.toml", "release-plz.toml", "cliff.toml"],
                ),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["release.yml"])),
            ("crates", dir_entry(&["example"], &[])),
            ("crates/example", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/example"]
resolver = "2"
"#,
            ),
            (
                "crates/example/Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
edition = "2024"
publish = false
"#,
            ),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);
    assertions::assert_unreadable_cached_files_fail_closed(&results);
}

#[test]
fn nested_crate_manifest_parse_failure_is_reported_without_suppressing_repo_results() {
    let root = temp_root("release-input-failures-nested-crate-parse");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &[".github", "crates"],
                    &["Cargo.toml", "release-plz.toml", "cliff.toml"],
                ),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["release.yml"])),
            ("crates", dir_entry(&["example"], &[])),
            ("crates/example", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/example"]
resolver = "2"
"#,
            ),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
"#,
            ),
            (
                "cliff.toml",
                r#"
[git]
conventional_commits = true
"#,
            ),
            (
                ".github/workflows/release.yml",
                r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz release-pr
"#,
            ),
            ("crates/example/Cargo.toml", "[package"),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            file: Some("crates/example/Cargo.toml"),
            ..ExpectedRuleResult::default()
        }],
    );
    assertions::assert_related_rule_results(
        &results,
        assertions::RELEASE_PLZ_EXISTS_RULE_ID,
        &[ExpectedRuleResult {
            file: Some("release-plz.toml"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
    assertions::assert_related_rule_results(
        &results,
        assertions::RELEASE_WORKFLOW_RULE_ID,
        &[ExpectedRuleResult {
            file: Some(".github/workflows/release.yml"),
            inventory: Some(true),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn nested_non_root_workflows_do_not_count_as_release_family_inputs() {
    let root = temp_root("release-input-failures-nested-workflow");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["examples"], &["Cargo.toml"])),
            ("examples", dir_entry(&["demo"], &[])),
            ("examples/demo", dir_entry(&[".github"], &[])),
            ("examples/demo/.github", dir_entry(&["workflows"], &[])),
            (
                "examples/demo/.github/workflows",
                dir_entry(&[], &["release.yml"]),
            ),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "examples/demo/.github/workflows/release.yml",
                "jobs:\n  release: [",
            ),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);

    assertions::assert_rule_file_absent(&results, "examples/demo/.github/workflows/release.yml");
}

#[test]
fn nested_non_root_release_configs_do_not_count_as_release_family_inputs() {
    let root = temp_root("release-input-failures-nested-configs");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["examples"],
                    &["Cargo.toml", "release-plz.toml", "cliff.toml"],
                ),
            ),
            ("examples", dir_entry(&["demo"], &[])),
            (
                "examples/demo",
                dir_entry(&[], &["release-plz.toml", "cliff.toml"]),
            ),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
"#,
            ),
            (
                "cliff.toml",
                r#"
[git]
conventional_commits = true
"#,
            ),
            ("examples/demo/release-plz.toml", "[workspace"),
            ("examples/demo/cliff.toml", "[git"),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);

    assertions::assert_rule_file_absent(&results, "examples/demo/release-plz.toml");
    assertions::assert_rule_file_absent(&results, "examples/demo/cliff.toml");
}

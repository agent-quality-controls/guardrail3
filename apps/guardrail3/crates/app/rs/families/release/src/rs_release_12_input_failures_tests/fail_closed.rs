use guardrail3_domain_report::Severity;

use super::super::super::check;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

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
    let failures = results
        .iter()
        .filter(|result| result.id == "RS-RELEASE-12")
        .collect::<Vec<_>>();

    assert_eq!(
        failures.len(),
        3,
        "expected exact input failures: {results:#?}"
    );
    assert!(
        failures
            .iter()
            .all(|result| result.severity == Severity::Error)
    );
    assert_eq!(
        failures
            .iter()
            .map(|result| result.file.as_deref())
            .collect::<Vec<_>>(),
        vec![
            Some(".github/workflows/release.yml"),
            Some("cliff.toml"),
            Some("release-plz.toml"),
        ]
    );
    assert!(!results.iter().any(|result| {
        result.id == "RS-RELEASE-03" && result.file.as_deref() == Some("release-plz.toml")
    }));
    assert!(!results.iter().any(|result| {
        result.id == "RS-RELEASE-04" && result.file.as_deref() == Some("cliff.toml")
    }));
    assert!(!results.iter().any(|result| {
        matches!(
            result.id.as_str(),
            "RS-RELEASE-05" | "RS-RELEASE-06" | "RS-RELEASE-07"
        ) && result.file.as_deref() == Some(".github/workflows/release.yml")
    }));
}

#[test]
fn cargo_parse_failure_is_reported_without_suppressing_other_repo_results() {
    let root = temp_root("release-input-failures-cargo");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github"], &["Cargo.toml", "release-plz.toml"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["release.yml"])),
        ],
        vec![
            ("Cargo.toml", "[package"),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
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
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);

    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-12" && result.file.as_deref() == Some("Cargo.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-02"
            && result.inventory
            && result.file.as_deref() == Some("release-plz.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-05"
            && result.inventory
            && result.file.as_deref() == Some(".github/workflows/release.yml")
    }));
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
        vec![],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);
    let failures = results
        .iter()
        .filter(|result| result.id == "RS-RELEASE-12")
        .map(|result| (result.file.as_deref(), result.message.as_str()))
        .collect::<Vec<_>>();

    assert!(failures.iter().any(|(file, message)| {
        *file == Some("Cargo.toml") && message.contains("Failed to read Cargo.toml")
    }));
    assert!(failures.iter().any(|(file, message)| {
        *file == Some("crates/example/Cargo.toml") && message.contains("Failed to read Cargo.toml")
    }));
    assert!(failures.iter().any(|(file, message)| {
        *file == Some("release-plz.toml") && message.contains("Failed to read release-plz.toml")
    }));
    assert!(failures.iter().any(|(file, message)| {
        *file == Some("cliff.toml") && message.contains("Failed to read cliff.toml")
    }));
    assert!(failures.iter().any(|(file, message)| {
        *file == Some(".github/workflows/release.yml")
            && message.contains("Failed to read workflow YAML")
    }));
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

    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-12"
            && result.file.as_deref() == Some("crates/example/Cargo.toml")
            && result.severity == Severity::Error
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-02"
            && result.inventory
            && result.file.as_deref() == Some("release-plz.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-RELEASE-05"
            && result.inventory
            && result.file.as_deref() == Some(".github/workflows/release.yml")
    }));
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

    assert!(!results.iter().any(|result| {
        result.id == "RS-RELEASE-12"
            && result.file.as_deref() == Some("examples/demo/.github/workflows/release.yml")
    }));
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

    assert!(!results.iter().any(|result| {
        result.id == "RS-RELEASE-12"
            && matches!(
                result.file.as_deref(),
                Some("examples/demo/release-plz.toml") | Some("examples/demo/cliff.toml")
            )
    }));
}

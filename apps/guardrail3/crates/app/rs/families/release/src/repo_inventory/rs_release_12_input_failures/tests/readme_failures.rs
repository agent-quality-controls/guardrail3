#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use guardrail3_app_rs_family_release_assertions::repo_inventory::rs_release_12_input_failures as assertions;

use super::helpers::{copy_fixture, run_family, write_file};

#[cfg(unix)]
#[test]
fn unreadable_readme_emits_input_failure_without_fake_quality_result() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/shared-types/Cargo.toml",
        r#"
[package]
name = "shared-types"
version = "0.1.0"
edition = "2024"
readme = "README.md"
"#,
    );
    write_file(
        tmp.path(),
        "packages/shared-types/README.md",
        "# Shared Types\n\nThis README should become unreadable before the release family runs.\n",
    );

    let readme_path = tmp.path().join("packages/shared-types/README.md");
    let original_permissions = std::fs::metadata(&readme_path)
        .expect("failed to read release README metadata")
        .permissions();
    let mut unreadable = original_permissions.clone();
    unreadable.set_mode(0o000);
    std::fs::set_permissions(&readme_path, unreadable)
        .expect("failed to make release README unreadable");

    let results = run_family(tmp.path(), false);

    std::fs::set_permissions(&readme_path, original_permissions)
        .expect("failed to restore release README permissions");

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("packages/shared-types/README.md"),
            message_contains: Some("Failed to read README"),
            ..Default::default()
        }],
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::README_QUALITY_RULE_ID,
        "packages/shared-types/README.md",
    );
}

#[cfg(unix)]
#[test]
fn unreadable_readme_for_non_publishable_crate_does_not_emit_input_failure() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/shared-types/Cargo.toml",
        r#"
[package]
name = "shared-types"
version = "0.1.0"
edition = "2024"
publish = false
readme = "README.md"
"#,
    );
    write_file(
        tmp.path(),
        "packages/shared-types/README.md",
        "# Shared Types\n\nThis README should become unreadable before the release family runs.\n",
    );

    let readme_path = tmp.path().join("packages/shared-types/README.md");
    let original_permissions = std::fs::metadata(&readme_path)
        .expect("failed to read release README metadata")
        .permissions();
    let mut unreadable = original_permissions.clone();
    unreadable.set_mode(0o000);
    std::fs::set_permissions(&readme_path, unreadable)
        .expect("failed to make release README unreadable");

    let results = run_family(tmp.path(), false);

    std::fs::set_permissions(&readme_path, original_permissions)
        .expect("failed to restore release README permissions");

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

#[cfg(unix)]
#[test]
fn unreadable_workspace_inherited_readme_emits_input_failure() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        r#"
[workspace]
members = ["packages/shared-types"]
resolver = "2"

[workspace.package]
readme = "README.md"
"#,
    );
    write_file(
        tmp.path(),
        "packages/shared-types/Cargo.toml",
        r#"
[package]
name = "shared-types"
version = "0.1.0"
edition = "2024"
description = "shared types"
license = "MIT"
repository = "https://example.com/shared-types"
readme.workspace = true
"#,
    );
    write_file(
        tmp.path(),
        "README.md",
        "# Workspace Readme\n\nThis README should become unreadable before the release family runs.\n",
    );

    let readme_path = tmp.path().join("README.md");
    let original_permissions = std::fs::metadata(&readme_path)
        .expect("failed to read release README metadata")
        .permissions();
    let mut unreadable = original_permissions.clone();
    unreadable.set_mode(0o000);
    std::fs::set_permissions(&readme_path, unreadable)
        .expect("failed to make release README unreadable");

    let results = run_family(tmp.path(), false);

    std::fs::set_permissions(&readme_path, original_permissions)
        .expect("failed to restore release README permissions");

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("README.md"),
            message_contains: Some("Failed to read README"),
            ..Default::default()
        }],
    );
    assertions::assert_related_rule_file_absent(
        &results,
        assertions::README_QUALITY_RULE_ID,
        "README.md",
    );
}

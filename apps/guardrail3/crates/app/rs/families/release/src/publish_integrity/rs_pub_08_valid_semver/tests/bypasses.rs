use super::super::check;
use super::super::run_tree as run_family;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::{crate_facts, crate_input};
use guardrail3_app_rs_family_release_assertions::publish_integrity::rs_pub_08_valid_semver as assertions;

#[test]
fn errors_on_invalid_semver_and_skips_non_publishable_crates() {
    let mut invalid = crate_facts("x");
    invalid.version_valid = false;
    invalid.version_string = Some("bad".to_owned());
    let invalid_input = crate_input(&invalid);
    let mut invalid_results = Vec::new();
    check(&invalid_input, &mut invalid_results);
    assert!(!assertions::findings(&invalid_results).is_empty());
    assertions::assert_rule_results(
        &invalid_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title_contains: Some("invalid semver"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(false),
            message_contains: Some("valid semver version or `version.workspace = true`"),
            ..Default::default()
        }],
    );

    let mut inherited_invalid = crate_facts("x");
    inherited_invalid.workspace_version = true;
    inherited_invalid.version_valid = false;
    inherited_invalid.version_string = None;
    let inherited_invalid_input = crate_input(&inherited_invalid);
    let mut inherited_invalid_results = Vec::new();
    check(&inherited_invalid_input, &mut inherited_invalid_results);
    assert!(!assertions::findings(&inherited_invalid_results).is_empty());
    assertions::assert_rule_results(
        &inherited_invalid_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title_contains: Some("invalid semver"),
            file: Some("crates/example/Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );

    let mut non_publishable = crate_facts("x");
    non_publishable.publishable = false;
    non_publishable.version_valid = false;
    let non_publishable_input = crate_input(&non_publishable);
    let mut non_publishable_results = Vec::new();
    check(&non_publishable_input, &mut non_publishable_results);
    assert!(assertions::findings(&non_publishable_results).is_empty());
    assertions::assert_rule_quiet(&non_publishable_results);
}

#[test]
fn errors_when_workspace_version_is_missing_or_invalid() {
    let missing_root = temp_root("release-workspace-version-missing");
    let missing_tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["pub"], &[])),
            ("crates/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/pub"]
resolver = "2"

[workspace.package]
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        missing_root,
    );
    let missing_results = run_family(&missing_tree, &StubToolChecker::new(true), false);
    assert!(!assertions::findings(&missing_results).is_empty());
    assertions::assert_rule_results(
        &missing_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("crates/pub/Cargo.toml"),
            ..Default::default()
        }],
    );

    let invalid_root = temp_root("release-workspace-version-invalid");
    let invalid_tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["pub"], &[])),
            ("crates/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/pub"]
resolver = "2"

[workspace.package]
version = "bad"
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        invalid_root,
    );
    let invalid_results = run_family(&invalid_tree, &StubToolChecker::new(true), false);
    assert!(!assertions::findings(&invalid_results).is_empty());
    assertions::assert_rule_results(
        &invalid_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("crates/pub/Cargo.toml"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_when_workspace_version_is_inherited_via_package_workspace_reference() {
    let root = temp_root("release-workspace-version-package-workspace");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &["Cargo.toml"])),
            ("packages", dir_entry(&["pub"], &[])),
            ("packages/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["packages/pub"]
resolver = "2"

[workspace.package]
version = "1.2.3"
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "packages/pub/Cargo.toml",
                r#"
[package]
name = "pub"
workspace = "../.."
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("packages/pub/Cargo.toml"),
            inventory: Some(true),
            message_contains: Some("`version.workspace = true`"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_package_workspace_reference_does_not_include_crate() {
    let root = temp_root("release-workspace-version-package-workspace-orphan");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &["Cargo.toml"])),
            ("packages", dir_entry(&["member", "orphan"], &[])),
            ("packages/member", dir_entry(&[], &["Cargo.toml"])),
            ("packages/orphan", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["packages/member"]
resolver = "2"

[workspace.package]
version = "1.2.3"
description = "shared"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "packages/member/Cargo.toml",
                r#"
[package]
name = "member"
version = "0.1.0"
edition = "2024"
publish = false
"#,
            ),
            (
                "packages/orphan/Cargo.toml",
                r#"
[package]
name = "orphan"
workspace = "../.."
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme = false
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("packages/orphan/Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_app_rs_family_release_assertions::rs_pub_03_repository_present as assertions;

#[test]
fn should_not_error_when_repository_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-repository-inheritance");
    let tree = project_tree(
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
version = "0.1.0"
description = "shared workspace description"
license = "MIT"
repository = "https://example.com/repo"
keywords = ["cli"]
categories = ["command-line-utilities"]
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
keywords.workspace = true
categories.workspace = true
readme = false
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("crates/pub/Cargo.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_ignore_non_member_crate_trying_to_inherit_workspace_repository() {
    let root = temp_root("release-workspace-repository-non-member");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["member", "orphan"], &[])),
            ("crates/member", dir_entry(&[], &["Cargo.toml"])),
            ("crates/orphan", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/member"]
resolver = "2"

[workspace.package]
version = "0.1.0"
repository = "https://example.com/repo"
"#,
            ),
            (
                "crates/member/Cargo.toml",
                r#"
[package]
name = "member"
version.workspace = true
edition = "2024"
publish = false
"#,
            ),
            (
                "crates/orphan/Cargo.toml",
                r#"
[package]
name = "orphan"
version = "0.1.0"
edition = "2024"
repository.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

#[test]
fn should_support_repository_inheritance_via_package_workspace_reference() {
    let root = temp_root("release-workspace-repository-package-workspace");
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
version = "0.1.0"
repository = "https://example.com/repo"
"#,
            ),
            (
                "packages/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version = "0.1.0"
edition = "2024"
workspace = "../.."
repository.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("packages/pub/Cargo.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_error_when_package_workspace_reference_does_not_include_crate() {
    let root = temp_root("release-workspace-repository-package-workspace-orphan");
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
version = "0.1.0"
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
version = "0.1.0"
edition = "2024"
workspace = "../.."
repository.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

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

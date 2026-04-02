use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_app_rs_family_release_assertions::publish_metadata::rs_pub_01_description_present as assertions;

#[test]
fn should_not_error_when_description_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-description-inheritance");
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
fn should_stay_out_of_scope_when_publish_is_inherited_false_from_workspace_package() {
    let root = temp_root("release-workspace-publish-false-inheritance");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["internal"], &[])),
            ("crates/internal", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/internal"]
resolver = "2"

[workspace.package]
version = "0.1.0"
publish = false
"#,
            ),
            (
                "crates/internal/Cargo.toml",
                r#"
[package]
name = "internal"
version.workspace = true
edition = "2024"
publish.workspace = true
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
fn should_stay_out_of_scope_when_publish_is_empty_array() {
    let root = temp_root("release-publish-empty-array");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml"])),
            ("crates", dir_entry(&["internal"], &[])),
            ("crates/internal", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/internal"]
resolver = "2"
"#,
            ),
            (
                "crates/internal/Cargo.toml",
                r#"
[package]
name = "internal"
version = "0.1.0"
edition = "2024"
publish = []
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
fn should_ignore_non_member_crate_trying_to_inherit_workspace_description() {
    let root = temp_root("release-workspace-description-non-member");
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
description = "shared workspace description"
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
description.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

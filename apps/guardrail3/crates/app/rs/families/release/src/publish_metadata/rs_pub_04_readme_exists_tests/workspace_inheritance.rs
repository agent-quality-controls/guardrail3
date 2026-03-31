use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_app_rs_family_release_assertions::rs_pub_04_readme_exists as assertions;

#[test]
fn should_inventory_when_readme_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-readme-inheritance");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "WORKSPACE.md"])),
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
readme = "WORKSPACE.md"
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
readme.workspace = true
"#,
            ),
            (
                "WORKSPACE.md",
                "# Shared README\n\nWorkspace-owned readme.\n",
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("WORKSPACE.md"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_resolve_workspace_readme_relative_to_workspace_root() {
    let root = temp_root("release-workspace-readme-relative-path");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &[])),
            ("packages", dir_entry(&["shared"], &["README.md"])),
            ("packages/shared", dir_entry(&["crates"], &["Cargo.toml"])),
            ("packages/shared/crates", dir_entry(&["pub"], &[])),
            (
                "packages/shared/crates/pub",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "packages/shared/Cargo.toml",
                r#"
[workspace]
members = ["crates/pub"]
resolver = "2"

[workspace.package]
version = "0.1.0"
description = "shared workspace description"
license = "MIT"
repository = "https://example.com/repo"
readme = "../README.md"
"#,
            ),
            (
                "packages/shared/crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
description.workspace = true
license.workspace = true
repository.workspace = true
readme.workspace = true
"#,
            ),
            (
                "README.md",
                "# Shared README\n\nWorkspace-relative readme.\n",
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("packages/shared/crates/pub/Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn should_ignore_non_member_crate_trying_to_inherit_workspace_readme() {
    let root = temp_root("release-workspace-readme-orphan");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "WORKSPACE.md"])),
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
readme = "WORKSPACE.md"
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
readme.workspace = true
"#,
            ),
            (
                "WORKSPACE.md",
                "# Shared README\n\nWorkspace-owned readme.\n",
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

#[test]
fn should_stay_quiet_when_readme_false_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-readme-false-inheritance");
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
readme = false
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
readme.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
}

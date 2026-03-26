use super::super::super::test_support::run_tree as check;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn should_not_warn_when_categories_are_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-categories-inheritance");
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
categories = ["command-line-utilities", "development-tools"]
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

    assert!(
        results.iter().any(|result| {
            result.id == "RS-PUB-07"
                && result.inventory
                && result.file.as_deref() == Some("crates/pub/Cargo.toml")
        }) && results.iter().all(|result| {
            !(result.id == "RS-PUB-07"
                && result.severity == guardrail3_domain_report::Severity::Warn
                && !result.inventory)
        }),
        "workspace-inherited categories should satisfy RS-PUB-07: {results:#?}"
    );
}

#[test]
fn should_warn_when_workspace_member_inherits_missing_workspace_categories() {
    let root = temp_root("release-workspace-categories-missing-value");
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
"#,
            ),
            (
                "crates/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version.workspace = true
edition = "2024"
categories.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-07"
            && result.severity == guardrail3_domain_report::Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("crates/pub/Cargo.toml")
            && result.title.contains("categories missing")
    }));
}

#[test]
fn should_warn_when_non_member_crate_tries_to_inherit_workspace_categories() {
    let root = temp_root("release-workspace-categories-orphan");
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
categories = ["command-line-utilities", "development-tools"]
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
categories.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-07"
            && result.severity == guardrail3_domain_report::Severity::Warn
            && !result.inventory
            && result.file.as_deref() == Some("crates/orphan/Cargo.toml")
    }));
}

#[test]
fn should_support_categories_inheritance_via_package_workspace_reference() {
    let root = temp_root("release-workspace-categories-package-workspace");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["ws", "packages"], &[])),
            ("ws", dir_entry(&[], &["Cargo.toml"])),
            ("packages", dir_entry(&["pub"], &[])),
            ("packages/pub", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "ws/Cargo.toml",
                r#"
[workspace]
members = ["../packages/pub"]
resolver = "2"

[workspace.package]
version = "0.1.0"
categories = ["command-line-utilities", "development-tools"]
"#,
            ),
            (
                "packages/pub/Cargo.toml",
                r#"
[package]
name = "pub"
version = "0.1.0"
edition = "2024"
workspace = "../../ws"
categories.workspace = true
"#,
            ),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().any(|result| {
            result.id == "RS-PUB-07"
                && result.inventory
                && result.file.as_deref() == Some("packages/pub/Cargo.toml")
        }) && results.iter().all(|result| {
            !(result.id == "RS-PUB-07"
                && result.severity == guardrail3_domain_report::Severity::Warn
                && !result.inventory)
        }),
        "package.workspace category inheritance should satisfy RS-PUB-07: {results:#?}"
    );
}

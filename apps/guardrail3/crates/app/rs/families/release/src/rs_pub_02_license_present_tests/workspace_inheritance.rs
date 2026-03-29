use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn should_not_error_when_license_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-license-inheritance");
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

    assert!(
        results
            .iter()
            .any(|result| result.id == "RS-PUB-02" && result.inventory),
        "workspace-inherited license should inventory under RS-PUB-02: {results:#?}"
    );
}

#[test]
fn should_not_error_when_license_file_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-license-file-inheritance");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "LICENSE-MIT"])),
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
license-file = "LICENSE-MIT"
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
license-file.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
readme = false
"#,
            ),
            ("LICENSE-MIT", "MIT text\n"),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(
        results
            .iter()
            .any(|result| result.id == "RS-PUB-02" && result.inventory),
        "workspace-inherited license-file should inventory under RS-PUB-02: {results:#?}"
    );
}

#[test]
fn should_error_when_non_member_crate_tries_to_inherit_workspace_license_file() {
    let root = temp_root("release-workspace-license-file-non-member");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["crates"], &["Cargo.toml", "LICENSE-MIT"])),
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
license-file = "LICENSE-MIT"
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
license-file.workspace = true
"#,
            ),
            ("LICENSE-MIT", "MIT text\n"),
        ],
        root,
    );
    let results = check(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-02"
            && result.severity == guardrail3_domain_report::Severity::Error
            && !result.inventory
            && result.file.as_deref() == Some("crates/orphan/Cargo.toml")
    }));
}

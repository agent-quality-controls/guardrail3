use super::super::super::check;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn should_not_warn_when_release_metadata_is_inherited_from_workspace_package() {
    let root = temp_root("release-workspace-internal-warning-inheritance");
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
        results.iter().all(|result| result.id != "RS-RELEASE-11"),
        "workspace-inherited metadata should prevent RS-RELEASE-11: {results:#?}"
    );
}

#[test]
fn should_not_warn_when_publish_is_inherited_empty_array_from_workspace_package() {
    let root = temp_root("release-workspace-publish-empty-array-inheritance");
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
publish = []
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

    assert!(
        results.iter().all(|result| result.id != "RS-RELEASE-11"),
        "workspace-inherited publish=[] should suppress RS-RELEASE-11: {results:#?}"
    );
}

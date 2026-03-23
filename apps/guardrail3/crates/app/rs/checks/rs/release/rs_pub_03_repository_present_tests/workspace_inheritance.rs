use super::super::super::check;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

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

    assert!(
        results.iter().all(|result| result.id != "RS-PUB-03"),
        "workspace-inherited repository should satisfy RS-PUB-03: {results:#?}"
    );
}

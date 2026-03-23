use super::super::super::check;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

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

    assert!(results.iter().any(|result| {
        result.id == "RS-PUB-04"
            && result.inventory
            && result.file.as_deref() == Some("WORKSPACE.md")
    }));
}

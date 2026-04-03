use super::helpers::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn scoped_release_run_ignores_cargo_manifests_outside_routed_workspace_surface() {
    let root = temp_root("release-input-failures-scoped-cargo-surface");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps", "tools"], &["release-plz.toml", "cliff.toml"]),
            ),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/api/crates", dir_entry(&["cli"], &[])),
            ("apps/api/crates/cli", dir_entry(&[], &["Cargo.toml"])),
            ("tools", dir_entry(&["helper"], &[])),
            ("tools/helper", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/api/Cargo.toml",
                r#"
[workspace]
members = ["crates/cli"]
resolver = "2"
"#,
            ),
            (
                "apps/api/crates/cli/Cargo.toml",
                r#"
[package]
name = "cli"
version = "0.1.0"
publish = false
"#,
            ),
            (
                "tools/helper/Cargo.toml",
                r#"
[package]
name = "helper"
version = "0.1.0"
"#,
            ),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
"#,
            ),
            ("cliff.toml", "[changelog]\nheader = \"# Changelog\"\n"),
        ],
        root,
    );
    let results = crate::test_fixtures::run_tree_with_validation_scope(
        &tree,
        &StubToolChecker::new(true),
        false,
        "apps/api",
    );

    assert!(
        !results
            .iter()
            .any(|result| result.file() == Some("tools/helper/Cargo.toml")),
        "scoped release routing should ignore Cargo.toml outside the routed workspace surface: {results:#?}"
    );
}

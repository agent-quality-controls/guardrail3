use super::super::run_tree_with_validation_scope;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn subtree_scope_keeps_repo_global_release_plz_coverage_strict() {
    let root = temp_root("release-plz-coverage-validation-scope");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["crates"], &["Cargo.toml", "release-plz.toml"]),
            ),
            ("crates", dir_entry(&["api", "cli"], &[])),
            (
                "crates/api",
                dir_entry(&["src"], &["Cargo.toml", "README.md"]),
            ),
            ("crates/api/src", dir_entry(&[], &["lib.rs"])),
            (
                "crates/cli",
                dir_entry(&["src"], &["Cargo.toml", "README.md"]),
            ),
            ("crates/cli/src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
members = ["crates/*"]
resolver = "2"
"#,
            ),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
release_always = false

[[package]]
name = "api"
"#,
            ),
            (
                "crates/api/Cargo.toml",
                r#"
[package]
name = "api"
version = "1.0.0"
edition = "2024"
description = "api"
license = "MIT"
repository = "https://example.com/api"
"#,
            ),
            ("crates/api/README.md", "# Api\n\nrelease docs\n"),
            ("crates/api/src/lib.rs", "pub fn api() {}\n"),
            (
                "crates/cli/Cargo.toml",
                r#"
[package]
name = "cli"
version = "1.0.0"
edition = "2024"
description = "cli"
license = "MIT"
repository = "https://example.com/cli"
"#,
            ),
            ("crates/cli/README.md", "# Cli\n\nrelease docs\n"),
            ("crates/cli/src/main.rs", "fn main() {}\n"),
        ],
        root,
    );

    let results =
        run_tree_with_validation_scope(&tree, &StubToolChecker::new(true), false, "crates/api/src");

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-RELEASE-03"
                && result.file() == Some("release-plz.toml")
                && result.title() == "release-plz missing crate `cli`"
        }),
        "{results:#?}"
    );
}

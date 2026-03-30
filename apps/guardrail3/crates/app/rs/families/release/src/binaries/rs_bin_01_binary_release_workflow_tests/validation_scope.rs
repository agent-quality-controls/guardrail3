use super::super::run_tree_with_validation_scope;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn subtree_scope_does_not_assume_unqualified_release_build_targets_only_live_binary() {
    let root = temp_root("release-bin-01-validation-scope");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github", "crates"], &["Cargo.toml", "LICENSE"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["binary-release.yml"])),
            ("crates", dir_entry(&["api", "cli"], &[])),
            (
                "crates/api",
                dir_entry(&["src"], &["Cargo.toml", "README.md"]),
            ),
            ("crates/api/src", dir_entry(&[], &["main.rs"])),
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
            ("LICENSE", "MIT\n"),
            (
                ".github/workflows/binary-release.yml",
                r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: softprops/action-gh-release@v2
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
            (
                "crates/api/README.md",
                "# Api\n\nThis binary is documented well enough for release checks.\n",
            ),
            ("crates/api/src/main.rs", "fn main() {}\n"),
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
            (
                "crates/cli/README.md",
                "# Cli\n\nThis binary is documented well enough for release checks.\n",
            ),
            ("crates/cli/src/main.rs", "fn main() {}\n"),
        ],
        root,
    );

    let results =
        run_tree_with_validation_scope(&tree, &StubToolChecker::new(true), false, "crates/api/src");

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-BIN-01"
                && result.file() == Some("crates/api/Cargo.toml")
                && result.title() == "api: no binary release workflow"
        }),
        "{results:#?}"
    );
}

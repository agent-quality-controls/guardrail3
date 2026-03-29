use super::super::run_tree as check;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};

#[test]
fn golden_config_only_tree_has_no_release_input_failures() {
    let root = temp_root("release-input-failures-golden");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &[".github"],
                    &["Cargo.toml", "release-plz.toml", "cliff.toml"],
                ),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["release.yml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            (
                "release-plz.toml",
                r#"
[workspace]
changelog_config = "cliff.toml"
"#,
            ),
            (
                "cliff.toml",
                r#"
[git]
conventional_commits = true
"#,
            ),
            (
                ".github/workflows/release.yml",
                r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz release-pr
"#,
            ),
        ],
        root,
    );
    let tool_checker = StubToolChecker::new(true);
    let results = check(&tree, &tool_checker, false);

    assert!(
        !results.iter().any(|result| result.id == "RS-RELEASE-12"),
        "unexpected input failures: {results:#?}"
    );
}

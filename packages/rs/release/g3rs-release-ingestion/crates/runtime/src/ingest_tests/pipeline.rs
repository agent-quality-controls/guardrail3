use std::fs;

use tempfile::tempdir;

use super::{crawl, git_init, long_readme, write};

#[test]
fn config_pipeline_reports_workflow_tooling_and_repo_inventory() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    let bin_dir = root.join("bin");
    fs::create_dir_all(&bin_dir).expect("create test bin dir");
    let tool_path = bin_dir.join("cargo-semver-checks");
    write(&tool_path, "#!/bin/sh\nexit 0\n");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mut permissions = fs::metadata(&tool_path)
            .expect("read tool metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&tool_path, permissions).expect("make tool executable");
    }

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/demo"]
resolver = "2"

[workspace.package]
publish = false

[profile.release]
opt-level = 3
"#,
    );
    write(
        root.join("crates/demo/Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
publish = false
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");
    write(
        root.join("release-plz.toml"),
        r#"
[workspace]
git_release_enable = true
release_always = false
changelog_config = "cliff.toml"
"#,
    );
    write(
        root.join("cliff.toml"),
        r#"
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
  { message = "^feat", group = "Features" },
  { message = "^fix", group = "Fixes" },
  { message = "^(doc|perf|refactor|style|test|chore)", group = "Other" },
]
"#,
    );
    write(
        root.join(".github/workflows/release.yml"),
        r#"
name: release
on:
  push:
    branches: [main]
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0
        with:
          command: release
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
      - run: cargo publish --dry-run --manifest-path crates/demo/Cargo.toml
"#,
    );

    let crawl = crawl(root);
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, Some(bin_dir.as_os_str()));
    let results = g3rs_release_config_checks::check(&input);

    for id in ["RS-RELEASE-CONFIG-15", "RS-RELEASE-CONFIG-16", "RS-RELEASE-CONFIG-17", "RS-RELEASE-CONFIG-21"] {
        assert!(results.iter().any(|result| result.id() == id), "{results:#?}");
    }
}

#[test]
fn filetree_pipeline_reports_missing_publishable_readme() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/demo"]
resolver = "2"
"#,
    );
    write(
        root.join("crates/demo/Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");

    let crawl = crawl(root);
    let input =
        crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_release_filetree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-RELEASE-FILETREE-04"
                && result.title() == "demo: README missing"
        }),
        "{results:#?}"
    );
}

#[cfg(unix)]
#[test]
fn source_pipeline_reports_unreadable_publishable_readme_without_quality_result() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/demo"]
resolver = "2"
"#,
    );
    write(
        root.join("crates/demo/Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");
    write(root.join("crates/demo/README.md"), &long_readme("Demo"));

    let readme_path = root.join("crates/demo/README.md");
    let original = super::make_unreadable(&readme_path);

    let crawl = crawl(root);
    let input = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_release_source_checks::check(&input);

    super::restore_permissions(&readme_path, original);

    assert!(
        results.iter().any(|result| result.id() == "RS-RELEASE-SOURCE-02"),
        "{results:#?}"
    );
    assert!(
        !results.iter().any(|result| result.id() == "RS-RELEASE-SOURCE-01"),
        "{results:#?}"
    );
}

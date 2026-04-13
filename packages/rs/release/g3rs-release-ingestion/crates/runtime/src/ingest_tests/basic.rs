use tempfile::tempdir;

use super::{crawl, git_init, long_readme, write};

#[test]
fn ingests_release_repo_and_publishable_readme_across_lanes() {
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
    write(root.join("LICENSE"), "MIT");
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
include = ["src/**", "Cargo.toml", "README.md", "LICENSE"]
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");
    write(root.join("crates/demo/README.md"), &long_readme("Demo"));

    let crawl = crawl(root);

    let config = crate::ingest_for_config_checks(&crawl).expect("config ingestion should succeed");
    assert_eq!(config.crates.len(), 1);
    assert_eq!(config.crates[0].name, "demo");
    assert!(config.crates[0].publishable);
    assert_eq!(config.repo.as_ref().unwrap().publishable_count, 1);
    assert!(config.repo.as_ref().unwrap().release_plz_exists);
    assert!(config.repo.as_ref().unwrap().cliff_exists);

    let filetree =
        crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    assert_eq!(filetree.repo.as_ref().unwrap().license_rel_path.as_deref(), Some("LICENSE"));
    assert!(filetree.repo.as_ref().unwrap().release_plz_exists);
    assert!(filetree.repo.as_ref().unwrap().cliff_exists);
    assert_eq!(filetree.readmes.len(), 1);
    assert_eq!(filetree.readmes[0].readme_rel_path, "crates/demo/README.md");
    assert!(filetree.readmes[0].readme_exists);

    let source =
        crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    assert_eq!(source.readmes.len(), 1);
    assert_eq!(source.readmes[0].readme_rel_path, "crates/demo/README.md");
    assert!(source.readmes[0].content.starts_with("# Demo"));
}

#[test]
fn ingests_workspace_inherited_readme_path() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/demo"]
resolver = "2"

[workspace.package]
readme = "README.md"
description = "workspace description"
license = "MIT"
repository = "https://example.com/workspace"
"#,
    );
    write(root.join("README.md"), &long_readme("Workspace"));
    write(
        root.join("crates/demo/Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
readme.workspace = true
description.workspace = true
license.workspace = true
repository.workspace = true
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");

    let crawl = crawl(root);

    let filetree =
        crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    assert_eq!(filetree.readmes.len(), 1);
    assert_eq!(filetree.readmes[0].readme_rel_path, "README.md");
    assert!(filetree.readmes[0].readme_exists);

    let source =
        crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    assert_eq!(source.readmes.len(), 1);
    assert_eq!(source.readmes[0].readme_rel_path, "README.md");
}

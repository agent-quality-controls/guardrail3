use std::path::Path;
use tempfile::tempdir;

use super::support::{crawl, git_init, long_readme, write};

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
publish = true
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

    let config = super::super::config_result(&crawl).expect("config ingestion should succeed");
    assert_eq!(config.crate_checks.len(), 1);
    assert_eq!(config.crate_checks[0].name, "demo");
    assert!(config.crate_checks[0].publishable);
    assert_eq!(config.repo_checks[0].publishable_count, 1);
    assert!(config.repo_checks[0].release_plz_exists);
    assert!(config.repo_checks[0].cliff_exists);

    let filetree =
        super::super::filetree_result(&crawl).expect("filetree ingestion should succeed");
    assert_eq!(filetree.repo.as_ref().unwrap().publishable_count, 1);
    assert_eq!(
        filetree.repo.as_ref().unwrap().license_rel_path.as_deref(),
        Some("LICENSE")
    );
    assert!(filetree.repo.as_ref().unwrap().release_plz_exists);
    assert!(filetree.repo.as_ref().unwrap().cliff_exists);
    assert_eq!(filetree.readmes.len(), 1);
    assert_eq!(filetree.readmes[0].readme_rel_path, "crates/demo/README.md");
    assert!(filetree.readmes[0].readme_exists);

    let source = super::super::source_result(&crawl).expect("source ingestion should succeed");
    assert_eq!(source.readmes.len(), 1);
    assert_eq!(source.readmes[0].readme_rel_path, "crates/demo/README.md");
    assert!(source.readmes[0].content.starts_with("# Demo"));
}

#[test]
fn non_workspace_root_fails_all_release_lanes() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
"#,
    );

    let crawl = crawl(root);

    assert!(super::super::config_result(&crawl).is_err());
    assert!(super::super::filetree_result(&crawl).is_err());
    assert!(super::super::source_result(&crawl).is_err());
}

#[test]
fn repo_root_checks_stub_returns_not_implemented() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);
    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = []
resolver = "2"
"#,
    );

    let crawl = crawl(root);
    let error = super::super::repo_root_result(&crawl).expect_err("stub should fail");

    assert!(matches!(
        error,
        crate::IngestionError::RepoRootChecksNotImplemented
    ));
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
publish = true
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
publish.workspace = true
readme.workspace = true
description.workspace = true
license.workspace = true
repository.workspace = true
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");

    let crawl = crawl(root);

    let filetree =
        super::super::filetree_result(&crawl).expect("filetree ingestion should succeed");
    assert_eq!(filetree.readmes.len(), 1);
    assert_eq!(filetree.readmes[0].readme_rel_path, "README.md");
    assert!(filetree.readmes[0].readme_exists);

    let source = super::super::source_result(&crawl).expect("source ingestion should succeed");
    assert_eq!(source.readmes.len(), 1);
    assert_eq!(source.readmes[0].readme_rel_path, "README.md");
}

#[test]
fn skips_release_burden_when_workspace_has_only_non_publishable_crates() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/public", "crates/internal"]
resolver = "2"
"#,
    );
    write(
        root.join("crates/public/Cargo.toml"),
        r#"
[package]
name = "public"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
internal = { path = "../internal", version = "0.1.0" }
"#,
    );
    write(
        root.join("crates/public/src/lib.rs"),
        "pub fn public() {}\n",
    );
    write(
        root.join("crates/internal/Cargo.toml"),
        r#"
[package]
name = "internal"
version = "0.1.0"
edition = "2024"
publish = false
"#,
    );
    write(
        root.join("crates/internal/src/lib.rs"),
        "pub fn internal() {}\n",
    );

    let crawl = crawl(root);

    let config = super::super::config_result(&crawl).expect("config ingestion should succeed");
    let config_results = g3rs_release_config_checks::check(&config);
    assert!(config_results.is_empty(), "{config_results:#?}");

    let filetree =
        super::super::filetree_result(&crawl).expect("filetree ingestion should succeed");
    assert_eq!(filetree.repo.as_ref().unwrap().publishable_count, 0);
    let filetree_results = g3rs_release_filetree_checks::check(&filetree);
    assert!(filetree_results.is_empty(), "{filetree_results:#?}");
}

#[test]
fn relative_nested_validate_path_does_not_fake_missing_manifest_in_dry_run() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("pkg/Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
publish = true
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
include = ["src/**", "Cargo.toml", "README.md"]

[workspace]
members = []
resolver = "2"
"#,
    );
    write(root.join("pkg/src/lib.rs"), "pub fn demo() {}\n");
    write(root.join("pkg/README.md"), &long_readme("Demo"));

    let original_dir = std::env::current_dir().expect("read current directory");
    std::env::set_current_dir(root).expect("enter fixture root");

    let crawl = crawl(Path::new("pkg"));
    let config = super::super::config_result(&crawl).expect("config ingestion should succeed");

    std::env::set_current_dir(original_dir).expect("restore current directory");

    let dry_run = config.crate_checks[0]
        .dry_run
        .as_ref()
        .expect("published crate should have dry-run result");

    match dry_run {
        g3rs_release_types::G3RsReleaseDryRunOutcome::Failed(stderr) => {
            assert!(
                !stderr.contains("manifest path `pkg/Cargo.toml` does not exist"),
                "{stderr}"
            );
        }
        g3rs_release_types::G3RsReleaseDryRunOutcome::Passed => {}
    }
}

use std::fs;

use g3rs_release_ingestion_assertions::ingest::collect as assertions;
use g3rs_release_types::G3RsReleaseConfigChecksInput;
use tempfile::tempdir;

#[cfg(unix)]
use super::support::make_unreadable;
use super::support::{crawl, git_init, long_readme, restore_permissions, write};

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
publish = true

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
publish.workspace = true
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
        r"
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
",
    );

    let crawl = crawl(root);
    let input = super::super::config_input_with_path(&crawl, Some(bin_dir.as_os_str()));
    let results = g3rs_release_config_checks::check(&input);

    for id in [
        "g3rs-release/semver-checks-installed",
        "g3rs-release/publish-status-inventory",
        "g3rs-release/release-profile-inventory",
        "g3rs-release/crate-inventory",
    ] {
        assert_eq!(assertions::count(&results, id), 1, "{results:#?}");
    }
}

#[test]
fn config_pipeline_detects_registry_token_from_job_env() {
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
publish = true
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
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
        r"
[git]
conventional_commits = true
",
    );
    write(
        root.join(".github/workflows/release.yml"),
        r"
name: release
on:
  push:
    branches: [main]
jobs:
  release:
    runs-on: ubuntu-latest
    env:
      CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    steps:
      - uses: release-plz/action@v0
        with:
          command: release
",
    );

    let crawl = crawl(root);
    let input = super::super::config_input_with_path(&crawl, None);

    assert_eq!(input.repos.len(), 1, "{input:#?}");
    assert!(
        input.repos[0].workflow_flags.has_registry_token_workflow,
        "{input:#?}"
    );
    assert_eq!(
        input.repos[0].registry_token_workflow_rel_path.as_deref(),
        Some(".github/workflows/release.yml"),
        "{input:#?}"
    );
}

#[test]
fn config_pipeline_detects_registry_token_from_workflow_env() {
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
publish = true
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");
    write(
        root.join(".github/workflows/release.yml"),
        r"
name: release
on:
  push:
    branches: [main]
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0
        with:
          command: release
",
    );

    let crawl = crawl(root);
    let input = super::super::config_input_with_path(&crawl, None);

    assert_eq!(input.repos.len(), 1, "{input:#?}");
    assert!(
        input.repos[0].workflow_flags.has_registry_token_workflow,
        "{input:#?}"
    );
    assert_eq!(
        input.repos[0].registry_token_workflow_rel_path.as_deref(),
        Some(".github/workflows/release.yml"),
        "{input:#?}"
    );
}

#[test]
fn config_pipeline_detects_publish_dry_run_through_cd_wrapper() {
    let input = input_with_publish_dry_run_wrapper(
        "cd x && cargo publish --dry-run --manifest-path crates/demo/Cargo.toml",
        ".github/workflows/release.yml",
    );

    assert_eq!(input.repos.len(), 1, "{input:#?}");
    assert!(
        input.repos[0].workflow_flags.has_publish_dry_run_workflow,
        "{input:#?}"
    );
    assert_eq!(
        input.repos[0].publish_dry_run_workflow_rel_path.as_deref(),
        Some(".github/workflows/release.yml"),
        "{input:#?}"
    );
}

#[test]
fn config_pipeline_detects_publish_dry_run_through_env_wrapper() {
    let input = input_with_publish_dry_run_wrapper(
        "env FOO=bar cargo publish --dry-run --manifest-path crates/demo/Cargo.toml",
        ".github/workflows/release.yml",
    );

    assert_eq!(input.repos.len(), 1, "{input:#?}");
    assert!(
        input.repos[0].workflow_flags.has_publish_dry_run_workflow,
        "{input:#?}"
    );
    assert_eq!(
        input.repos[0].publish_dry_run_workflow_rel_path.as_deref(),
        Some(".github/workflows/release.yml"),
        "{input:#?}"
    );
}

#[test]
fn config_pipeline_detects_publish_dry_run_through_shell_wrapper() {
    let input = input_with_publish_dry_run_wrapper(
        "sh -c 'cargo publish --dry-run --manifest-path crates/demo/Cargo.toml'",
        ".github/workflows/release.yml",
    );

    assert_eq!(input.repos.len(), 1, "{input:#?}");
    assert!(
        input.repos[0].workflow_flags.has_publish_dry_run_workflow,
        "{input:#?}"
    );
    assert_eq!(
        input.repos[0].publish_dry_run_workflow_rel_path.as_deref(),
        Some(".github/workflows/release.yml"),
        "{input:#?}"
    );
}

fn input_with_publish_dry_run_wrapper(
    run_line: &str,
    workflow_rel_path: &str,
) -> G3RsReleaseConfigChecksInput {
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
publish = true
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");
    let workflow = format!(
        r"
name: release
on:
  push:
    branches: [main]
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: {run_line}
"
    );
    write(root.join(workflow_rel_path), &workflow);

    let crawl = crawl(root);
    super::super::config_input_with_path(&crawl, None)
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
publish = true
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");

    let crawl = crawl(root);
    let input = super::super::filetree_result(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_release_filetree_checks::check(&input);

    assertions::assert_present(
        &results,
        "g3rs-release/readme-exists",
        "demo: README missing",
        Some("crates/demo/Cargo.toml"),
        false,
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
publish = true
description = "demo crate"
license = "MIT"
repository = "https://example.com/demo"
readme = "README.md"
"#,
    );
    write(root.join("crates/demo/src/lib.rs"), "pub fn demo() {}\n");
    write(root.join("crates/demo/README.md"), &long_readme("Demo"));

    let readme_path = root.join("crates/demo/README.md");
    let original = make_unreadable(&readme_path);

    let crawl = crawl(root);
    let input = super::super::source_result(&crawl).expect("source ingestion should succeed");
    let results = g3rs_release_source_checks::check(&input);

    restore_permissions(&readme_path, original);

    assertions::assert_present(
        &results,
        "g3rs-release/source-input-failures",
        "failed to read release source input",
        Some("crates/demo/README.md"),
        false,
    );
    assertions::assert_no_results(&results, "g3rs-release/readme-quality");
}

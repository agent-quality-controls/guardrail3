use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

#[test]
fn ingests_with_both_cargo_and_clippy() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let output = result
        .expect("ingestion should succeed when both Cargo.toml and clippy.toml are present");
    assert_eq!(
        output.dependency.cargo_rel_path, "Cargo.toml",
        "dependency input should reference the root Cargo.toml"
    );
    assert!(
        output.dependency.cargo.workspace.is_some(),
        "parsed Cargo.toml should contain a [workspace] section"
    );
    let clippy_bans = output
        .clippy_bans
        .as_ref()
        .expect("clippy_bans should be Some when clippy.toml exists");
    assert_eq!(
        clippy_bans.clippy_rel_path, "clippy.toml",
        "clippy_bans input should reference the root clippy.toml"
    );
}

#[test]
fn ingests_with_dot_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let output = result.expect("ingestion should succeed with .clippy.toml variant");
    let clippy_bans = output
        .clippy_bans
        .as_ref()
        .expect("clippy_bans should be Some when .clippy.toml exists");
    assert_eq!(
        clippy_bans.clippy_rel_path, ".clippy.toml",
        "clippy_bans should reference .clippy.toml when only the dotfile variant exists"
    );
}

#[test]
fn clippy_bans_is_none_without_clippy_config() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let output = result
        .expect("ingestion should succeed even without clippy config (it is optional)");
    assert_eq!(
        output.dependency.cargo_rel_path, "Cargo.toml",
        "dependency input should still be present without clippy config"
    );
    assert!(
        output.clippy_bans.is_none(),
        "clippy_bans should be None when no clippy config file exists"
    );
}

#[test]
fn fails_when_cargo_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::CargoTomlNotFound)),
        "ingestion should return CargoTomlNotFound when Cargo.toml is missing even if clippy.toml exists"
    );
}

#[test]
fn fails_on_malformed_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::ParseFailed { .. })),
        "ingestion should return ParseFailed when Cargo.toml contains invalid TOML"
    );
}

#[test]
fn malformed_clippy_toml_produces_none_not_error() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(root.join("clippy.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let output = result.expect(
        "ingestion should succeed even with malformed clippy.toml (clippy config is optional)",
    );
    assert!(
        output.clippy_bans.is_none(),
        "clippy_bans should be None when clippy.toml fails to parse"
    );
}

#[test]
fn ignored_but_recovered_cargo_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "Cargo.toml\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"recovered\"\n");

    let crawl = crawl(root);

    let crawl_entry = crawl
        .entry("Cargo.toml")
        .expect("Cargo.toml should be present in crawl via recovery even when gitignored");
    assert_eq!(
        crawl_entry.ignore_state,
        g3rs_workspace_crawl::G3RsWorkspaceIgnoreState::Ignored,
        "Cargo.toml should have Ignored state when gitignored, proving recovery path was exercised"
    );

    let result = crate::ingest(&crawl);
    let output = result.expect(
        "ingestion should succeed for a gitignored Cargo.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        output.dependency.cargo_rel_path, "Cargo.toml",
        "recovered Cargo.toml should still resolve to the root-relative path"
    );
}

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
    let result = crate::ingest_config(&crawl);

    let input = result
        .expect("ingestion should succeed when both Cargo.toml and clippy.toml are present");
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should reference the root Cargo.toml"
    );
    assert!(
        input.cargo.workspace.is_some(),
        "parsed Cargo.toml should contain a [workspace] section"
    );
    assert_eq!(
        input.clippy_rel_path.as_deref(),
        Some("clippy.toml"),
        "clippy_rel_path should reference the root clippy.toml when present"
    );
    assert!(
        input.clippy.is_some(),
        "clippy should be Some when clippy.toml exists and parses successfully"
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
    let result = crate::ingest_config(&crawl);

    let input = result.expect("ingestion should succeed with .clippy.toml variant");
    assert_eq!(
        input.clippy_rel_path.as_deref(),
        Some(".clippy.toml"),
        "clippy_rel_path should reference .clippy.toml when only the dotfile variant exists"
    );
}

#[test]
fn clippy_is_none_without_clippy_config() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crawl(root);
    let result = crate::ingest_config(&crawl);

    let input = result
        .expect("ingestion should succeed even without clippy config (it is optional)");
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should still be present without clippy config"
    );
    assert!(
        input.clippy.is_none(),
        "clippy should be None when no clippy config file exists"
    );
    assert!(
        input.clippy_rel_path.is_none(),
        "clippy_rel_path should be None when no clippy config file exists"
    );
}

#[test]
fn fails_when_cargo_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest_config(&crawl);

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
    let result = crate::ingest_config(&crawl);

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
    let result = crate::ingest_config(&crawl);

    let input = result.expect(
        "ingestion should succeed even with malformed clippy.toml (clippy config is optional)",
    );
    assert!(
        input.clippy.is_none(),
        "clippy should be None when clippy.toml fails to parse"
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

    let result = crate::ingest_config(&crawl);
    let input = result.expect(
        "ingestion should succeed for a gitignored Cargo.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "recovered Cargo.toml should still resolve to the root-relative path"
    );
}

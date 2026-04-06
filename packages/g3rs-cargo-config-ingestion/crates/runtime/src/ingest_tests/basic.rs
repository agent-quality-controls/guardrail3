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
fn ingests_valid_workspace_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write(root.join("src/lib.rs"), "");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect("ingestion should succeed for a valid Cargo.toml workspace");
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should be the workspace-root-relative path"
    );
    assert!(
        input.cargo.workspace.is_some(),
        "parsed Cargo.toml should contain a workspace section when [workspace] is present"
    );
}

#[test]
fn fails_when_cargo_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("src/lib.rs"), "");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::CargoTomlNotFound)
        ),
        "ingestion should return CargoTomlNotFound when no Cargo.toml exists in the workspace"
    );
}

#[test]
fn fails_on_malformed_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "{{{{not valid toml at all}}}}");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ParseFailed { .. })
        ),
        "ingestion should return ParseFailed when Cargo.toml contains invalid TOML"
    );
}

#[test]
fn ingests_package_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect("ingestion should succeed for a valid package Cargo.toml");
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should be the workspace-root-relative path"
    );
    let package = input
        .cargo
        .package
        .as_ref()
        .expect("parsed Cargo.toml should have a [package] section when one is defined");
    assert_eq!(
        package.name.as_deref(),
        Some("demo"),
        "parsed package name should match the fixture"
    );
}

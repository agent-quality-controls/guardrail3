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
fn ingests_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect("ingestion should succeed for a valid clippy.toml");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy_rel_path should be the workspace-root-relative path"
    );
    assert_eq!(
        input.clippy.msrv.as_deref(),
        Some("1.85"),
        "parsed ClippyToml should contain the msrv value from the fixture file"
    );
}

#[test]
fn ingests_dot_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect("ingestion should succeed for a valid .clippy.toml");
    assert_eq!(
        input.clippy_rel_path, ".clippy.toml",
        "clippy_rel_path should be the dot-prefixed variant when only .clippy.toml exists"
    );
    assert_eq!(
        input.clippy.msrv.as_deref(),
        Some("1.85"),
        "parsed ClippyToml should contain the msrv value from the dot-prefixed fixture file"
    );
}

#[test]
fn prefers_clippy_toml_over_dot_variant() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.80\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect("ingestion should succeed when both clippy config variants exist");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy.toml should be preferred over .clippy.toml when both exist"
    );
    assert_eq!(
        input.clippy.msrv.as_deref(),
        Some("1.85"),
        "parsed content should come from clippy.toml (1.85), not .clippy.toml (1.80)"
    );
}

#[test]
fn fails_when_clippy_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ClippyTomlNotFound)
        ),
        "ingestion should return ClippyTomlNotFound when no clippy config exists in the workspace"
    );
}

#[test]
fn fails_on_malformed_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "{{{{not valid toml at all}}}}");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ParseFailed { .. })
        ),
        "ingestion should return ParseFailed when clippy.toml contains invalid TOML"
    );
}

#[test]
fn fails_on_unknown_fields() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "totally_fake_field = true\n",
    );

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ParseFailed { .. })
        ),
        "ingestion should return ParseFailed when clippy.toml contains unknown fields (deny_unknown_fields)"
    );
}

#[test]
fn fails_on_wrong_value_type() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = 42\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ParseFailed { .. })
        ),
        "ingestion should return ParseFailed when a field has the wrong TOML type (integer instead of string)"
    );
}

#[test]
fn empty_clippy_toml_parses_to_defaults() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect("ingestion should succeed for an empty clippy.toml");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy_rel_path should be the workspace-root-relative path"
    );
    assert!(
        input.clippy.msrv.is_none(),
        "empty clippy.toml should have no msrv set"
    );
}

#[test]
fn nested_clippy_toml_is_not_selected() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("subdir/clippy.toml"),
        "msrv = \"1.85\"\n",
    );

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ClippyTomlNotFound)
        ),
        "ingestion should not select a clippy.toml in a subdirectory, only at the workspace root"
    );
}

#[test]
fn ignored_but_recovered_clippy_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "clippy.toml\n");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    let input = result.expect(
        "ingestion should succeed for a gitignored clippy.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "recovered clippy.toml should still resolve to the root-relative path"
    );
    assert_eq!(
        input.clippy.msrv.as_deref(),
        Some("1.85"),
        "recovered clippy.toml should be parsed correctly"
    );
}

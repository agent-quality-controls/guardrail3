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

const CARGO_TOML: &str = "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n";
const RELEASE_PLZ_TOML: &str =
    "[workspace]\nchangelog_config = \"cliff.toml\"\ngit_release_enable = true\nrelease_always = false\n";
const CLIFF_TOML: &str = "[git]\nconventional_commits = true\nfilter_unconventional = true\n";

// ---------------------------------------------------------------------------
// 1. All three files present
// ---------------------------------------------------------------------------

#[test]
fn ingests_all_three_files() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), CARGO_TOML);
    write(root.join("release-plz.toml"), RELEASE_PLZ_TOML);
    write(root.join("cliff.toml"), CLIFF_TOML);

    let crawl = crawl(root);
    let input = crate::ingest_config(&crawl)
        .expect("ingestion should succeed when all three config files are present");

    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should be the workspace-root-relative path to Cargo.toml"
    );
    assert!(
        input.release_plz_rel_path.is_some(),
        "release_plz_rel_path should be Some when release-plz.toml is present"
    );
    assert_eq!(
        input.release_plz_rel_path.as_deref(),
        Some("release-plz.toml"),
        "release_plz_rel_path should be 'release-plz.toml'"
    );
    assert!(
        input.release_plz.is_some(),
        "release_plz should be Some when release-plz.toml is present and valid"
    );
    assert!(
        input.cliff_rel_path.is_some(),
        "cliff_rel_path should be Some when cliff.toml is present"
    );
    assert_eq!(
        input.cliff_rel_path.as_deref(),
        Some("cliff.toml"),
        "cliff_rel_path should be 'cliff.toml'"
    );
    assert!(
        input.cliff.is_some(),
        "cliff should be Some when cliff.toml is present and valid"
    );
}

// ---------------------------------------------------------------------------
// 2. Without release-plz.toml
// ---------------------------------------------------------------------------

#[test]
fn ingests_without_release_plz() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), CARGO_TOML);
    write(root.join("cliff.toml"), CLIFF_TOML);

    let crawl = crawl(root);
    let input = crate::ingest_config(&crawl)
        .expect("ingestion should succeed without release-plz.toml");

    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert!(
        input.release_plz_rel_path.is_none(),
        "release_plz_rel_path should be None when release-plz.toml is absent"
    );
    assert!(
        input.release_plz.is_none(),
        "release_plz should be None when release-plz.toml is absent"
    );
    assert!(
        input.cliff.is_some(),
        "cliff should be Some when cliff.toml is present"
    );
}

// ---------------------------------------------------------------------------
// 3. Without cliff.toml
// ---------------------------------------------------------------------------

#[test]
fn ingests_without_cliff() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), CARGO_TOML);
    write(root.join("release-plz.toml"), RELEASE_PLZ_TOML);

    let crawl = crawl(root);
    let input = crate::ingest_config(&crawl)
        .expect("ingestion should succeed without cliff.toml");

    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert!(
        input.release_plz.is_some(),
        "release_plz should be Some when release-plz.toml is present"
    );
    assert!(
        input.cliff_rel_path.is_none(),
        "cliff_rel_path should be None when cliff.toml is absent"
    );
    assert!(
        input.cliff.is_none(),
        "cliff should be None when cliff.toml is absent"
    );
}

// ---------------------------------------------------------------------------
// 4. Cargo.toml only
// ---------------------------------------------------------------------------

#[test]
fn ingests_cargo_only() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), CARGO_TOML);

    let crawl = crawl(root);
    let input = crate::ingest_config(&crawl)
        .expect("ingestion should succeed with only Cargo.toml");

    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert!(
        input.release_plz_rel_path.is_none(),
        "release_plz_rel_path should be None when release-plz.toml is absent"
    );
    assert!(
        input.release_plz.is_none(),
        "release_plz should be None when release-plz.toml is absent"
    );
    assert!(
        input.cliff_rel_path.is_none(),
        "cliff_rel_path should be None when cliff.toml is absent"
    );
    assert!(
        input.cliff.is_none(),
        "cliff should be None when cliff.toml is absent"
    );
}

// ---------------------------------------------------------------------------
// 5. Fails when Cargo.toml is missing
// ---------------------------------------------------------------------------

#[test]
fn fails_when_cargo_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    // Write something else so the crawl has entries but no Cargo.toml
    write(root.join("release-plz.toml"), RELEASE_PLZ_TOML);

    let crawl = crawl(root);
    let result = crate::ingest_config(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::CargoTomlNotFound)),
        "ingestion should return CargoTomlNotFound when no Cargo.toml exists in the workspace"
    );
}

// ---------------------------------------------------------------------------
// 6. Fails on malformed Cargo.toml
// ---------------------------------------------------------------------------

#[test]
fn fails_on_malformed_cargo() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let result = crate::ingest_config(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::ParseFailed { .. })),
        "ingestion should return ParseFailed when Cargo.toml contains invalid TOML"
    );
}

// ---------------------------------------------------------------------------
// 7. Malformed release-plz.toml produces None, not error
// ---------------------------------------------------------------------------

#[test]
fn malformed_release_plz_produces_none() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), CARGO_TOML);
    write(root.join("release-plz.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let input = crate::ingest_config(&crawl)
        .expect("ingestion should succeed even when release-plz.toml is malformed — graceful degradation");

    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert!(
        input.release_plz_rel_path.is_none(),
        "release_plz_rel_path should be None when release-plz.toml is malformed"
    );
    assert!(
        input.release_plz.is_none(),
        "release_plz should be None when release-plz.toml is malformed"
    );
}

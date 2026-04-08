use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_code_ast_ingestion_assertions::{assert_source_file, require_source_file};
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn ingests_owned_rust_files_and_classifies_tests() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/lib.rs"), "pub fn run() {}\n");
    write(root.join("src/http_tests.rs"), "pub fn helper() {}\n");
    write(root.join("tests/smoke.rs"), "#[test]\nfn smoke() {}\n");
    write(root.join("tests/fixtures/probe.rs"), "fn fixture() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 3, "fixture file should be excluded");

    assert_source_file(
        require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        None,
        "pub fn run() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "src/http_tests.rs"),
        "src/http_tests.rs",
        true,
        None,
        "pub fn helper() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "tests/smoke.rs"),
        "tests/smoke.rs",
        true,
        None,
        "#[test]\nfn smoke() {}\n",
    );
}

#[test]
fn ingested_inputs_drive_code_ast_checks() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/lib.rs"), "pub fn run() { todo!(); }\n");
    write(
        root.join("tests/smoke.rs"),
        "#[test]\nfn smoke() { panic!(\"boom\"); }\n",
    );

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    let lib_results = g3rs_code_ast_checks::check(require_source_file(&inputs, "src/lib.rs"));
    let test_results = g3rs_code_ast_checks::check(require_source_file(&inputs, "tests/smoke.rs"));

    assert!(
        lib_results.iter().any(|result| result.id() == "RS-CODE-13"),
        "lib input should preserve todo! detection"
    );
    assert!(
        test_results.is_empty(),
        "test-owned source should preserve current no-findings behavior for the migrated rules"
    );
}

#[cfg(unix)]
#[test]
fn unreadable_selected_source_fails_ingestion() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    let secret = root.join("src/secret.rs");
    write(&secret, "pub fn hidden() {}\n");

    let mut permissions = fs::metadata(&secret)
        .expect("metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&secret, permissions).expect("chmod should succeed");

    let workspace_crawl = crawl(root).expect("crawl should succeed even with unreadable files");
    let error = crate::ingest_for_ast_checks(&workspace_crawl)
        .expect_err("selected unreadable source should fail ingestion");

    assert!(
        matches!(error, crate::IngestionError::Unreadable { .. }),
        "unexpected error: {error:?}"
    );
}

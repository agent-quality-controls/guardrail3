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
        false,
        "pub fn run() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "src/http_tests.rs"),
        "src/http_tests.rs",
        true,
        None,
        false,
        "pub fn helper() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "tests/smoke.rs"),
        "tests/smoke.rs",
        true,
        None,
        false,
        "#[test]\nfn smoke() {}\n",
    );
}

#[test]
fn classifies_library_root_and_library_module() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "mod helper;\npub fn run() {}\n");
    write(root.join("src/helper.rs"), "pub struct Helper;\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_source_file(
        require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        Some("library"),
        true,
        "mod helper;\npub fn run() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "src/helper.rs"),
        "src/helper.rs",
        false,
        Some("library"),
        false,
        "pub struct Helper;\n",
    );
}

#[test]
fn classifies_binary_root_in_mixed_package() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "pub fn api() {}\n");
    write(root.join("src/main.rs"), "fn main() {}\n");
    write(root.join("src/bin/tool.rs"), "fn main() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_source_file(
        require_source_file(&inputs, "src/lib.rs"),
        "src/lib.rs",
        false,
        Some("library"),
        true,
        "pub fn api() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "src/main.rs"),
        "src/main.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "src/bin/tool.rs"),
        "src/bin/tool.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
}

#[test]
fn classifies_custom_library_root_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/custom_lib.rs\"\n",
    );
    write(root.join("src/custom_lib.rs"), "pub fn api() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_source_file(
        require_source_file(&inputs, "src/custom_lib.rs"),
        "src/custom_lib.rs",
        false,
        Some("library"),
        true,
        "pub fn api() {}\n",
    );
}

#[test]
fn classifies_explicit_binary_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[[bin]]\nname = \"tool\"\npath = \"src/tool.rs\"\n",
    );
    write(root.join("src/tool.rs"), "fn main() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_source_file(
        require_source_file(&inputs, "src/tool.rs"),
        "src/tool.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
}

#[test]
fn classifies_nested_workspace_member_ownership() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\n",
    );
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/core/src/lib.rs"), "pub fn api() {}\n");
    write(root.join("crates/core/src/main.rs"), "fn main() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_source_file(
        require_source_file(&inputs, "crates/core/src/lib.rs"),
        "crates/core/src/lib.rs",
        false,
        Some("library"),
        true,
        "pub fn api() {}\n",
    );
    assert_source_file(
        require_source_file(&inputs, "crates/core/src/main.rs"),
        "crates/core/src/main.rs",
        false,
        Some("binary"),
        false,
        "fn main() {}\n",
    );
}

#[test]
fn leaves_unowned_source_without_profile() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("tools/probe.rs"), "pub fn probe() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&workspace_crawl).expect("ingestion should succeed");

    assert_source_file(
        require_source_file(&inputs, "tools/probe.rs"),
        "tools/probe.rs",
        false,
        None,
        false,
        "pub fn probe() {}\n",
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

#[test]
fn malformed_nearest_cargo_toml_fails_ingestion() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    write(root.join("src/lib.rs"), "pub fn run() {}\n");

    let workspace_crawl = crawl(root).expect("crawl should succeed");
    let error = crate::ingest_for_ast_checks(&workspace_crawl)
        .expect_err("malformed owning Cargo.toml should fail ingestion");

    assert!(
        matches!(error, crate::IngestionError::ParseFailed { .. }),
        "unexpected error: {error:?}"
    );
}

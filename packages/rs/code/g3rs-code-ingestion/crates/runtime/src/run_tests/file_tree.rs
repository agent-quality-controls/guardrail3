#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures need direct filesystem and process access to build temp workspaces"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_code_ingestion_assertions::run as assertions;
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
fn file_tree_input_supports_glob_members_and_excludes() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/*\"]\n\
exclude = [\"crates/skip\"]\n\
resolver = \"2\"\n",
    );
    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/api/src/lib.rs"), "");
    write(
        root.join("crates/skip/Cargo.toml"),
        "[package]\nname = \"skip\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/skip/src/lib.rs"), "");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

    assertions::assert_root_cargo_paths(&input, &["crates/api/Cargo.toml"]);
}

#[test]
fn file_tree_input_supports_owned_roots_with_no_rust_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

    assertions::assert_root_cargo_paths(&input, &["crates/api/Cargo.toml"]);
}

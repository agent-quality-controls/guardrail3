use std::process::Command;
use std::{fs, path::Path};

use g3rs_workspace_crawl_assertions::workspace_queries::{
    assert_extension_count, assert_root_file_exists,
};
use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

#[test]
fn supports_basic_queries_over_the_crawl() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);
    fs::create_dir_all(root.join("src")).expect("create source directory for query test");
    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("src/lib.rs"), "pub fn demo() {}\n");
    write(root.join("src/main.rs"), "fn main() {}\n");
    write(root.join("README.md"), "# demo\n");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    assert_root_file_exists(&crawl, "Cargo.toml");
    assert_extension_count(&crawl, "rs", 2);
}

fn write(path: impl AsRef<Path>, content: &str) {
    fs::write(path, content).expect("write query fixture file");
}

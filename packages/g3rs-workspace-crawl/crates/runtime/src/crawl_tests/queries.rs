use std::{fs, path::Path};

use g3rs_workspace_crawl_assertions::workspace_queries::{
    assert_extension_count, assert_root_file_exists,
};
use tempfile::tempdir;

#[test]
fn supports_basic_queries_over_the_crawl() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
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

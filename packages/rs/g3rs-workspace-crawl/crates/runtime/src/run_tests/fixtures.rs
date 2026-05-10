//! Shared test fixtures for the `run_tests` integration suite.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Initialise a git repository at `path` with no output.
pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

/// Write `content` to `path`, creating any missing parent directories.
pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

/// Write a minimal `Cargo.toml` to `root` so that `crawl` accepts the path
/// as a workspace root.
pub(super) fn write_root_manifest(root: &Path) {
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
}

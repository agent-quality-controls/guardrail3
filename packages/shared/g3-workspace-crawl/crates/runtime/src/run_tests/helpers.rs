//! Shared filesystem fixture helpers for run-test sidecars.
#![allow(
    clippy::disallowed_methods,
    reason = "fixture helpers must call std::fs and Command directly to seed test workspaces"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

/// Initializes a quiet git repository at `path` so the `ignore` crate can resolve
/// gitignore semantics during the crawl under test.
pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

/// Writes `content` to `path`, creating parent directories as needed.
pub(super) fn write_fixture(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}
